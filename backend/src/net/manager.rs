use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
};

use pocker_back::{ResponseWrapper, base::table::PlayerState, casino};
use tokio::sync::{Mutex, RwLock};

use crate::net::dispatcher::{BatchMsg, DispatcherCmd, FatMsg};

pub struct Manager {
    playes_addr: Arc<RwLock<HashMap<String, SocketAddr>>>,
    addr_playes: Arc<RwLock<HashMap<SocketAddr, String>>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            playes_addr: Arc::new(RwLock::new(HashMap::new())),
            addr_playes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn process(&mut self, fat: FatMsg) -> Vec<BatchMsg> {
        let from = fat.from.unwrap();
        let mut batch_msgs = vec![];

        for cmd in self.handle_command(&fat.msg).await {
            let resp = match cmd {
                ResponseWrapper::TableId(_) => BatchMsg::new(vec![from.clone()], cmd.into()),
                ResponseWrapper::UserId(id) => {
                    self.playes_addr.write().await.insert(id.clone(), from);

                    self.addr_playes.write().await.insert(from, id.clone());

                    BatchMsg::new(vec![from], ResponseWrapper::UserId(id).into())
                }
                ResponseWrapper::Players(list) => {
                    let players = self.playes_addr.read().await;

                    let recceivers: Vec<SocketAddr> = list
                        .iter()
                        .map(|p| players.get(&p.id).unwrap().clone())
                        .collect();

                    BatchMsg::new(recceivers, ResponseWrapper::Players(list).into())
                }
                ResponseWrapper::PlayerStateChanged(_, state) => {
                    match self.addr_playes.read().await.get(&from) {
                        Some(player_id) => {
                            let players = self.playes_addr.read().await;

                            let recceivers = casino::player_change_state(player_id, &state)
                                .await
                                .unwrap()
                                .iter()
                                .map(|p| players.get(&p.id).unwrap().clone())
                                .collect();

                            BatchMsg::new(
                                recceivers,
                                ResponseWrapper::PlayerStateChanged(player_id.clone(), state)
                                    .into(),
                            )
                        }
                        None => BatchMsg::new(
                            vec![from.clone()],
                            ResponseWrapper::Unknown("can't change state!".to_string()).into(),
                        ),
                    }
                }
                ResponseWrapper::Unknown(_) => BatchMsg::new(vec![from.clone()], cmd.into()),
                ResponseWrapper::PlayerDisconnected(_) => BatchMsg::new(vec![], cmd.into()),
            };
            batch_msgs.push(resp);
        }
        batch_msgs
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
                    ResponseWrapper::PlayerDisconnected(player_id).into(),
                )]
            }
        }
    }

    async fn handle_command(&mut self, msg: &str) -> Vec<ResponseWrapper> {
        let cmd: Vec<&str> = msg.split("::").collect();

        match cmd.len() {
            1 => match cmd[0] {
                "set_a_table" => vec![ResponseWrapper::TableId(casino::set_a_table().await)],
                "READY" => {
                    vec![ResponseWrapper::PlayerStateChanged(
                        String::new(),
                        PlayerState::READY,
                    )]
                }
                _ => vec![ResponseWrapper::Unknown(msg.to_string())],
            },
            2 => match cmd[0] {
                "add_player_to_table" => {
                    let table_id = cmd[1];
                    let player_id = casino::add_player_to_table(table_id).await.unwrap();
                    let players = casino::get_table_players(table_id)
                        .await
                        .unwrap()
                        .into_iter()
                        .collect();
                    vec![
                        ResponseWrapper::UserId(player_id),
                        ResponseWrapper::Players(players),
                    ]
                }
                _ => vec![ResponseWrapper::Unknown(msg.to_string())],
            },
            _ => vec![ResponseWrapper::Unknown(msg.to_string())],
        }
    }
}
