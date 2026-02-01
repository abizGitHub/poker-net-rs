use crate::base::casino;
use common::*;

pub struct StateManager;

impl StateManager {
    pub fn new() -> Self {
        StateManager {}
    }

    pub async fn process(&mut self, player_id: &str, state: &PlayerState) -> (TableDto, bool) {
        casino::player_change_state(player_id, &state).await
    }
}
