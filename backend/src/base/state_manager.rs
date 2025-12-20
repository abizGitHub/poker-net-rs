use crate::base::{
    casino,
    table::{PlayerState, TableDto},
};

pub struct StateManager;

impl StateManager {
    pub fn new() -> Self {
        StateManager {}
    }

    pub async fn process(
        &mut self,
        player_id: &str,
        state: &PlayerState,
    ) -> Vec<StateManagerResponse> {
        let table = casino::player_change_state(player_id, &state)
            .await
            .unwrap();

        let state_changed = StateManagerResponse::PlayerStateChanged(table.clone());

        if table.players.iter().all(|f| f.state == PlayerState::READY) {
            vec![state_changed, StateManagerResponse::StartGame(table)]
        } else {
            vec![state_changed]
        }
    }
}

pub enum StateManagerResponse {
    PlayerStateChanged(TableDto),
    StartGame(TableDto),
}
