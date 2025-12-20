use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
};

mod base;
mod net;
use crate::net::dispatcher::{Dispatcher, handle_socket};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("WebSocket server running at ws://{addr}");

    // track active sockets if needed
    let active_clients: Arc<Mutex<HashSet<SocketAddr>>> =
        Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));

    let dispatcher = RwLock::new(Dispatcher::new());

    loop {
        let (stream, peer_addr) = listener.accept().await.unwrap();
        let tx = dispatcher.read().await.get_sender_ws();
        let rx = dispatcher.write().await.get_receiver_ws(peer_addr).await;
        let tx_dspch = dispatcher.read().await.get_sender_dispatcher();

        let clients = active_clients.clone();

        tokio::spawn(
            async move { handle_socket(stream, peer_addr, tx, rx, clients, tx_dspch).await },
        );
    }
}
