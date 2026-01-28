use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    base::{
        card::Card,
        casino,
        state_manager::StateManager,
        table::{GameResult, GameState, PlayerDto, PlayerState},
    },
    net::dispatcher::{BatchMsg, DispatcherCmd, FatMsg},
};

pub struct Manager {
    playes_addr: Arc<RwLock<HashMap<String, SocketAddr>>>,
    addr_playes: Arc<RwLock<HashMap<SocketAddr, String>>>,
    state_manager: StateManager,
}

impl Manager {
    pub fn new() -> Self {
        let playes_addr = Arc::new(RwLock::new(HashMap::new()));
        Manager {
            playes_addr,
            addr_playes: Arc::new(RwLock::new(HashMap::new())),
            state_manager: StateManager::new(),
        }
    }

    pub async fn process(&mut self, fat: FatMsg) -> Vec<BatchMsg> {
        let from = fat.from.unwrap();

        match RequestWrapper::from(fat.msg.as_str()) {
            RequestWrapper::SetATable => {
                vec![BatchMsg::new(
                    vec![from.clone()],
                    ResponseWrapper::TableId(casino::set_a_table().await),
                )]
            }

            RequestWrapper::AddPlayerToTable(table_id) => {
                let player_id = casino::add_player_to_table(&table_id).await.unwrap();

                let players: Vec<PlayerDto> = casino::get_table_players(&table_id)
                    .await
                    .unwrap()
                    .into_iter()
                    .collect();

                self.playes_addr
                    .write()
                    .await
                    .insert(player_id.clone(), from);

                self.addr_playes
                    .write()
                    .await
                    .insert(from, player_id.clone());

                let recceivers: Vec<SocketAddr> = self.players_to_address(&players).await;

                vec![
                    BatchMsg::new(vec![from.clone()], ResponseWrapper::UserId(player_id)),
                    BatchMsg::new(recceivers, ResponseWrapper::Players(players)),
                ]
            }

            RequestWrapper::Ready => match self.addr_playes.read().await.get(&from) {
                Some(player_id) => {
                    let (table, state_changed) = self
                        .state_manager
                        .process(player_id, &PlayerState::READY)
                        .await;

                    let receivers = self.players_to_address(&table.players).await;

                    let mut batches = vec![BatchMsg::new(
                        receivers.clone(),
                        ResponseWrapper::Players(table.players),
                    )];
                    if state_changed {
                        batches.push(BatchMsg::new(
                            receivers.clone(),
                            ResponseWrapper::GameStatusChanged(table.state.clone()),
                        ));
                        match table.state {
                            GameState::Flop | GameState::Turn | GameState::River => {
                                batches.push(BatchMsg::new(
                                    receivers.clone(),
                                    ResponseWrapper::CardsOnTable(table.card_on_table),
                                ))
                            }
                            GameState::Ended => batches.push(BatchMsg::new(
                                receivers.clone(),
                                ResponseWrapper::GameFinished(
                                    table.result.expect("game has finished!"),
                                ),
                            )),
                            _ => {}
                        }
                    }
                    batches
                }

                None => vec![BatchMsg::new(
                    vec![from.clone()],
                    ResponseWrapper::Unknown(fat.msg.to_string()),
                )],
            },

            RequestWrapper::AllTables => {
                let all_tables = casino::all_tables().await;
                println!("<all_tables>{all_tables:?}");
                vec![BatchMsg::new(
                    vec![from.clone()],
                    ResponseWrapper::AllTables(all_tables),
                )]
            }

            RequestWrapper::Unknown(msg) => vec![BatchMsg::new(
                vec![from.clone()],
                ResponseWrapper::Unknown(msg),
            )],
        }
    }

    async fn players_to_address(&self, players: &Vec<PlayerDto>) -> Vec<SocketAddr> {
        let map = self.playes_addr.read().await;
        players
            .iter()
            .map(|p| map.get(&p.id).unwrap().clone())
            .collect()
    }

    pub async fn dispatcher_cmd(&mut self, cmd: DispatcherCmd) -> Vec<BatchMsg> {
        match cmd {
            DispatcherCmd::PlayerLeft(addr) => {
                if let Some(player_id) = self.addr_playes.write().await.remove(&addr) {
                    let players_on_table = casino::player_disconnected(&player_id).await;
                    let players_map = self.playes_addr.read().await;

                    let players_on_table = players_on_table
                        .iter()
                        .map(|p| players_map.get(&p.id).unwrap().clone())
                        .collect();

                    vec![BatchMsg::new(
                        players_on_table,
                        ResponseWrapper::PlayerDisconnected(player_id),
                    )]
                } else {
                    vec![]
                }
            }
        }
    }
}

pub enum ResponseWrapper {
    TableId(String),
    UserId(String),
    Players(Vec<PlayerDto>),
    PlayerDisconnected(String),
    CardsOnTable(Vec<Card>),
    GameStatusChanged(GameState),
    GameFinished(GameResult),
    AllTables(Vec<String>),
    Unknown(String),
}

impl Into<String> for ResponseWrapper {
    fn into(self) -> String {
        match self {
            Self::TableId(id) => format!("table_id::{id}"),
            Self::UserId(id) => format!("user_id::{id}"),
            Self::Players(ps) => match serde_json::to_string(&ps) {
                Ok(s) => format!("players::{s}"),
                Err(_) => format!("error in players!"),
            },
            Self::PlayerDisconnected(id) => format!("player_disconnected::{id}"),
            Self::GameStatusChanged(status) => match serde_json::to_string(&status) {
                Ok(s) => format!("game::{s}"),
                Err(_) => format!("error in game!"),
            },
            Self::CardsOnTable(cards) => format!("table::{:?}", cards),
            Self::GameFinished(result) => match serde_json::to_string(&result) {
                Ok(s) => format!("end::{s}"),
                Err(_) => format!("error in end!"),
            },
            Self::AllTables(tables) => format!("all_tables::{:?}", tables),
            Self::Unknown(m) => format!("unknown::{m}"),
        }
    }
}

pub enum RequestWrapper {
    SetATable,
    AddPlayerToTable(String),
    Ready,
    AllTables,
    Unknown(String),
}

impl From<&str> for RequestWrapper {
    fn from(value: &str) -> Self {
        let cmd: Vec<&str> = value.split("::").collect();
        match cmd.len() {
            1 => match cmd[0] {
                "set_a_table" => Self::SetATable,
                "ready" => Self::Ready,
                "all_tables" => Self::AllTables,
                _ => Self::Unknown(value.to_string()),
            },
            2 => match cmd[0] {
                "add_player_to_table" => Self::AddPlayerToTable(cmd[1].to_string()),
                _ => Self::Unknown(value.to_string()),
            },
            _ => Self::Unknown(value.to_string()),
        }
    }
}
