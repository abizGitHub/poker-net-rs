use crate::base::{
    casino,
    table::{GameResult, PlayerState, TableDto},
};

pub struct StateManager;

impl StateManager {
    pub fn new() -> Self {
        StateManager {}
    }

    pub async fn process(&mut self, player_id: &str, state: &PlayerState) -> (TableDto, bool) {
        casino::player_change_state(player_id, &state)
            .await
            .unwrap()
    }

    pub async fn get_result(&self, table_id: &str) -> GameResult {
        casino::get_table_result(table_id).await
    }
}
