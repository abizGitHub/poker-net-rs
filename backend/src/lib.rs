pub use crate::base::casino;

pub mod base;

pub enum ResponseWrapper {
    TableId(String),
    UserId(String),
    Players(Vec<String>),
    PlayerDisconnected(String),
    Unknown(String),
}

impl Into<String> for ResponseWrapper {
    fn into(self) -> String {
        match self {
            Self::TableId(id) => format!("table_id::{id}"),
            Self::UserId(id) => format!("user_id::{id}"),
            Self::Players(ps) => format!("players::{}", ps.join(",")),
            Self::PlayerDisconnected(id) => format!("player_discannected::{id}"),
            Self::Unknown(m) => format!("unknown::{m}"),
        }
    }
}
