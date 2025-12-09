use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::Message;

use crate::base::casino;
mod base;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001")
        .await
        .expect("Cannot bind");

    println!("WebSocket server running at ws://127.0.0.1:9001");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut ws = accept_async(stream).await.unwrap();
            println!("Client connected");
            while let Some(msg) = ws.next().await {
                let msg = msg.unwrap();
                println!("{}", msg);
                ws.send(Message::Text(handle_command(&msg.into_text().unwrap()).await))
                    .await
                    .unwrap();
            }
        });
    }
}

async fn handle_command(msg: &str) -> String {
    let cmd: Vec<&str> = msg.split("::").collect();
    println!("{msg}");  
    match cmd.len() {
        1 => match cmd[0] {
            "set_a_table" => casino::set_a_table().await,
            _ => format!("{msg} not found!"),
        },
        2 => match cmd[0] {
            "add_player_to_table" => casino::add_player_to_table(cmd[1]).await.unwrap(),
            "get_table_players" => casino::get_table_players(cmd[1]).await.unwrap().join(","),
            _ => format!("table({}) not found!", cmd[1]),
        },
        _ => format!("{msg} not found!"),
    }
}
