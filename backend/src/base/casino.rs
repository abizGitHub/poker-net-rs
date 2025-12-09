use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::base::table::GameTable;
static DB: Lazy<RwLock<HashMap<String, GameTable>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn set_a_table() -> String {
    let table_id = generate_long_id();
    let table = GameTable::set_a_table();
    DB.write()
        .expect("could not write!")
        .insert(table_id.clone(), table);
    table_id
}

pub async fn add_player_to_table(table_id: &str) -> Result<String, ()> {
    match DB.write().unwrap().get_mut(table_id) {
        Some(table) => {
            let palyer_id = generate_short_id();
            table.add_player(&palyer_id);
            Ok(palyer_id)
        }
        None => Err(()),
    }
}

pub async fn get_table_players(table_id: &str) -> Result<Vec<String>, ()> {
    match DB.read().unwrap().get(table_id) {
        Some(table) => Ok(table.players()),
        None => Err(()),
    }
}

fn generate_short_id() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .split("-")
        .next()
        .unwrap()
        .to_string()
}

fn generate_long_id() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .split("-")
        .into_iter()
        .skip(4)
        .next()
        .unwrap()
        .to_string()
}
