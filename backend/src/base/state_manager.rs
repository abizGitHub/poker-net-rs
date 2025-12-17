use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::{ResponseWrapper, base::table::PlayerState, casino};

pub struct StateManager {
    playes_addr: Arc<RwLock<HashMap<String, SocketAddr>>>,
}

impl StateManager {
    pub fn new(playes_addr: Arc<RwLock<HashMap<String, SocketAddr>>>) -> Self {
        StateManager { playes_addr }
    }

    pub async fn process(&mut self, player_id: &str, state: &PlayerState) -> Vec<ResponseWrapper> {
        let start_game = casino::player_change_state(player_id, &state)
            .await
            .unwrap()
            .iter()
            .all(|f| f.state == PlayerState::READY);

        let state_changed =
            ResponseWrapper::PlayerStateChanged(player_id.to_string(), state.clone());

        if start_game {
            vec![state_changed, ResponseWrapper::StartGame]
        } else {
            vec![state_changed]
        }
    }
}
