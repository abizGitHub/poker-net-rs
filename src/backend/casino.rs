use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;

use crate::backend::game_base::GameTable;

static TABLES: Lazy<RwLock<HashMap<String, GameTable>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn set_a_table() -> String {
    let table_id = rand::random::<usize>().to_string();
    let table = GameTable::set_a_table();
    TABLES.write().unwrap().insert(table_id.clone(), table);
    table_id
}
