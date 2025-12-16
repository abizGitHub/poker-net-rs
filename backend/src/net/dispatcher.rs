use futures_util::{SinkExt, StreamExt};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    net::TcpStream,
    sync::{
        Mutex, RwLock,
        broadcast::{self, Receiver, Sender},
    },
};
use tokio_tungstenite::accept_async;
use tungstenite::Message;

use crate::net::manager::Manager;

#[derive(Debug, Clone)]
pub struct FatMsg {
    pub from: Option<SocketAddr>,
    pub msg: String,
}

#[derive(Debug, Clone)]
pub struct BatchMsg {
    to: Vec<SocketAddr>,
    msg: String,
}

impl BatchMsg {
    pub fn new(to: Vec<SocketAddr>, msg: String) -> Self {
        BatchMsg { to, msg }
    }
}

impl FatMsg {
    pub fn from_mngr(msg: String) -> Self {
        FatMsg { from: None, msg }
    }
    pub fn new(addr: SocketAddr, msg: String) -> Self {
        FatMsg {
            from: Some(addr),
            msg,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DispatcherCmd {
    PlayerLeft(SocketAddr),
}

pub struct Dispatcher {
    pub tx_self: Sender<DispatcherCmd>,
    tx_ws: Sender<FatMsg>,
    outbox: Arc<Mutex<HashMap<SocketAddr, Sender<Message>>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let outbox = Arc::new(Mutex::new(HashMap::<SocketAddr, Sender<Message>>::new()));
        let (tx_ws, mut rx_ws) = broadcast::channel::<FatMsg>(1);

        let (tx_self, mut rx_self) = broadcast::channel::<DispatcherCmd>(1);

        let manager = Arc::new(RwLock::new(Manager::new()));
        let outbox_clone = outbox.clone();
        let outbox_clone2 = outbox.clone();
        let manager_clone = manager.clone();

        tokio::spawn(async move {
            let manager_clone = manager_clone.clone();

            while let Ok(msg) = rx_ws.recv().await {
                let mut manager = manager_clone.write().await;
                let outbox = outbox_clone.lock().await;

                for resp in manager.process(msg).await {
                    for id in resp.to {
                        let _ = outbox
                            .get(&id)
                            .unwrap()
                            .send(Message::Text(resp.msg.clone()));
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Ok(msg) = rx_self.recv().await {
                match msg {
                    DispatcherCmd::PlayerLeft(addr) => {
                        outbox_clone2.lock().await.remove(&addr);
                    }
                }
                let outbox = outbox_clone2.lock().await;
                for resp in manager.write().await.dispatcher_cmd(msg).await {
                    resp.to.iter().for_each(|addr| {
                        let _ = outbox
                            .get(addr)
                            .unwrap()
                            .send(Message::Text(resp.msg.clone()));
                    });
                }
            }
        });

        Dispatcher {
            tx_self,
            tx_ws,
            outbox,
        }
    }

    pub fn get_sender_ws(&self) -> Sender<FatMsg> {
        self.tx_ws.clone()
    }

    pub fn get_sender_dispatcher(&self) -> Sender<DispatcherCmd> {
        self.tx_self.clone()
    }

    pub async fn get_receiver_ws(&mut self, addr: SocketAddr) -> Receiver<Message> {
        let (sender, _) = broadcast::channel::<Message>(10);
        self.outbox.lock().await.insert(addr, sender.clone());
        sender.subscribe()
    }
}

pub async fn handle_socket(
    stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<FatMsg>,
    mut rx: Receiver<Message>,
    clients: Arc<Mutex<HashSet<SocketAddr>>>,
    tx_dispatcher: Sender<DispatcherCmd>,
) {
    clients.lock().await.insert(addr);

    let ws_stream = accept_async(stream).await.unwrap();
    println!("New connection from {addr}");

    let (mut write, mut read) = ws_stream.split();

    // task: forward inbound socket messages into broadcast channel
    //let tx_inbound = tx.clone();
    let inbound = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(_) | Message::Binary(_)) => {
                    let _ = tx.send(FatMsg::new(addr, msg.unwrap().into_text().unwrap()));
                }
                Ok(Message::Close(_)) => {
                    let _ = tx.send(FatMsg::new(addr, "left..".to_string()));
                    break;
                }
                _ => {}
            }
        }
    });

    // task: receive broadcast and send to this client
    let outbound = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if write.send(msg).await.is_err() {
                break;
            }
        }
    });

    inbound.await.unwrap();
    outbound.abort(); // stop outbound when client disconnects

    clients.lock().await.remove(&addr);
    let _ = tx_dispatcher.send(DispatcherCmd::PlayerLeft(addr));
    println!("Client disconnected: {addr}");
}
