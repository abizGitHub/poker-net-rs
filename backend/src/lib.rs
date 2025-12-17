use serde::{Deserialize, Serialize};

pub use crate::base::casino;
use crate::base::table::{PlayerState, Role};

pub mod base;
pub mod net;

pub enum RequestWrapper {
    SetATable,
    AddPlayerToTable(String),
    ReadyToStartGame,
    Unknown(String),
}

impl From<&str> for RequestWrapper {
    fn from(value: &str) -> Self {
        let cmd: Vec<&str> = value.split("::").collect();
        match cmd.len() {
            1 => match cmd[0] {
                "set_a_table" => Self::SetATable,
                "READY" => Self::ReadyToStartGame,
                _ => Self::Unknown(value.to_string()),
            },
            2 => match cmd[0] {
                "add_player_to_table" => Self::AddPlayerToTable(cmd[1].to_string()),
                _ => Self::Unknown(value.to_string()),
            },
            _ => Self::Unknown(value.to_string()),
        }
    }
}

pub enum ResponseWrapper {
    TableId(String),
    UserId(String),
    Players(Vec<PlayerDto>),
    PlayerDisconnected(String),
    PlayerStateChanged(String, PlayerState),
    StartGame,
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
