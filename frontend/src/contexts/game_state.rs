use common::{Card, GameState, Player};

#[derive(PartialEq, Clone, Debug, Default)]
pub struct ContextHolder {
    pub game_state: GameState,
    pub table_id: String,
    pub user_id: String,
    pub players: Vec<Player>,
    pub cards_on_table: Vec<Card>,
}
