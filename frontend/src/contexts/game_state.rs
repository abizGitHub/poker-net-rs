#[derive(PartialEq, Default, Clone, Debug)]
pub struct ContextHolder {
    pub game_state: String,
    pub table_id: String,
    pub user_id: String,
    pub players: Vec<String>,
}
