use serde::{Deserialize, Serialize};

pub use crate::base::casino;
use crate::base::table::{PlayerState, Role};

pub mod base;

pub enum ResponseWrapper {
    TableId(String),
    UserId(String),
    Players(Vec<PlayerDto>),
    PlayerDisconnected(String),
    PlayerStateChanged(String, PlayerState),
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDto {
    pub id: String,
    role: Option<Role>,
    state: PlayerState,
}

impl Into<String> for ResponseWrapper {
    fn into(self) -> String {
        match self {
            Self::TableId(id) => format!("table_id::{id}"),
            Self::UserId(id) => format!("user_id::{id}"),
            Self::Players(ps) => {
                let f = serde_json::to_string(&ps);
                println!("{f:?}");
                format!("players::{:?}", f)
            }
            Self::PlayerDisconnected(id) => format!("player_discannected::{id}"),
            Self::PlayerStateChanged(id, state) => format!("player::{id}::changed::{state:?}"),
            Self::Unknown(m) => format!("unknown::{m}"),
        }
    }
}
