use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::base::table::{GameTable, PlayerDto, PlayerState, TableDto};
static TABLES: Lazy<RwLock<HashMap<String, GameTable>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static PLAYERS_ON_TABLES: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn set_a_table() -> String {
    let table_id = generate_long_id();
    let table = GameTable::set_a_table();
    TABLES.write().await.insert(table_id.clone(), table);
    table_id
}

pub async fn add_player_to_table(table_id: &str) -> Result<String, ()> {
    match TABLES.write().await.get_mut(table_id) {
        Some(table) => {
            let palyer_id = generate_short_id();
            table.add_player(&palyer_id);

            PLAYERS_ON_TABLES
                .write()
                .await
                .insert(palyer_id.clone(), table_id.to_string());
            Ok(palyer_id)
        }
        None => Err(()),
    }
}

pub async fn get_table_players(table_id: &str) -> Result<Vec<PlayerDto>, ()> {
    match TABLES.read().await.get(table_id) {
        Some(table) => Ok(table.players()),
        None => Err(()),
    }
}

pub async fn player_disconnected(player_id: &str) -> Vec<PlayerDto> {
    let table_id = PLAYERS_ON_TABLES.write().await.remove(player_id).unwrap();

    match TABLES.write().await.get_mut(&table_id) {
        Some(table) => {
            table.remove_player(player_id);
            table.players().iter().map(|p| p.clone()).collect()
        }
        None => vec![],
    }
}

pub async fn player_change_state(player_id: &str, new_state: &PlayerState) -> (TableDto, bool) {
    let table_id = {
        PLAYERS_ON_TABLES
            .read()
            .await
            .get(player_id)
            .unwrap()
            .to_string()
    };
    let (table, game_changed) = {
        let mut lock = TABLES.write().await;
        let game_changed = lock
            .get_mut(&table_id)
            .unwrap()
            .player_change_state(player_id, new_state);

        (lock.get(&table_id).unwrap().clone(), game_changed)
    };

    (
        TableDto::from(
            &table_id,
            &table,
            TABLES.read().await.get(&table_id).unwrap().get_result(),
        ),
        game_changed,
    )
}

pub async fn all_tables() -> Vec<String> {
    TABLES.read().await.keys().map(|k| k.clone()).collect()
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
