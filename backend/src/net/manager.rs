use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    RequestWrapper, ResponseWrapper,
    base::{state_manager::StateManager, table::PlayerState},
    casino,
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
        let cloned = playes_addr.clone();
        Manager {
            playes_addr,
            addr_playes: Arc::new(RwLock::new(HashMap::new())),
            state_manager: StateManager::new(cloned),
        }
    }

    pub async fn process(&mut self, fat: FatMsg) -> Vec<BatchMsg> {
        let from = fat.from.unwrap();
        let mut batch_msgs = vec![];

        let responses = match RequestWrapper::from(fat.msg.as_str()) {
            RequestWrapper::SetATable => {
                vec![ResponseWrapper::TableId(casino::set_a_table().await)]
            }
            RequestWrapper::AddPlayerToTable(table_id) => {
                let player_id = casino::add_player_to_table(&table_id).await.unwrap();
                let players = casino::get_table_players(&table_id)
                    .await
                    .unwrap()
                    .into_iter()
                    .collect();

                vec![
                    ResponseWrapper::UserId(player_id),
                    ResponseWrapper::Players(players),
                ]
            }

            RequestWrapper::ReadyToStartGame => match self.addr_playes.read().await.get(&from) {
                Some(player_id) => {
                    self.state_manager
                        .process(player_id, &PlayerState::READY)
                        .await
                }
                None => vec![ResponseWrapper::Unknown(fat.msg.to_string())],
            },

            RequestWrapper::Unknown(msg) => vec![ResponseWrapper::Unknown(fat.msg.to_string())],
        };

        for response in responses {
            let resp = match response {
                ResponseWrapper::TableId(_) => BatchMsg::new(vec![from.clone()], response.into()),
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

                            let recceivers: Vec<SocketAddr> = list
                                .iter()
                                .map(|p| players.get(&p.id).unwrap().clone())
                                .collect();

                            BatchMsg::new(recceivers, ResponseWrapper::Players(list).into())
                        }
                        None => BatchMsg::new(vec![from.clone()], "".to_string()),
                    }
                }
                ResponseWrapper::StartGame => {}
                ResponseWrapper::PlayerDisconnected(_) => BatchMsg::new(vec![], response.into()),
                ResponseWrapper::Unknown(_) => BatchMsg::new(vec![from.clone()], response.into()),
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
}
