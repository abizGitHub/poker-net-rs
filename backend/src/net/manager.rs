use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    base::{
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
                let player_id = self.addr_playes.write().await.remove(&addr).unwrap();
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
            }
        }
    }
}

pub enum ResponseWrapper {
    TableId(String),
    UserId(String),
    Players(Vec<PlayerDto>),
    PlayerDisconnected(String),
    GameStatusChanged(GameState),
    GameFinished(GameResult),
    Unknown(String),
}

impl Into<String> for ResponseWrapper {
    fn into(self) -> String {
        match self {
            Self::TableId(id) => format!("table_id::{id}"),
            Self::UserId(id) => format!("user_id::{id}"),
            Self::Players(ps) => {
                let f = serde_json::to_string(&ps);
                println!("{f:?}");
                format!("players::{:?}", f)
            }
            Self::PlayerDisconnected(id) => format!("player_discannected::{id}"),
            Self::GameStatusChanged(status) => format!("game::{status:?}"),
            Self::GameFinished(result) => format!("end::{result:?}"),
            Self::Unknown(m) => format!("unknown::{m}"),
        }
    }
}

pub enum RequestWrapper {
    SetATable,
    AddPlayerToTable(String),
    Ready,
    Unknown(String),
}

impl From<&str> for RequestWrapper {
    fn from(value: &str) -> Self {
        let cmd: Vec<&str> = value.split("::").collect();
        match cmd.len() {
            1 => match cmd[0] {
                "set_a_table" => Self::SetATable,
                "ready" => Self::Ready,
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
