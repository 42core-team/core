use tokio::{
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use super::{bridge::bridge, Message};

#[derive(Debug)]
pub struct BridgeCon {
    pub id: u64,
    pub sender: Option<Sender<Message>>,
    pub receiver: Option<Receiver<Message>>,
    pub disconnect: Option<Receiver<()>>,
    is_disconnected: bool,
}

impl BridgeCon {
    pub fn new(stream: TcpStream) -> Self {
        let (sender, receiver, disconnect) = bridge(stream);
        Self {
            id: 0,
            sender: Some(sender),
            receiver: Some(receiver),
            disconnect: Some(disconnect),
            is_disconnected: false,
        }
    }

    pub fn new_fake() -> Self {
        Self {
            id: 0,
            sender: None,
            receiver: None,
            disconnect: None,
            is_disconnected: false,
        }
    }

    pub fn is_disconnected(&mut self) -> bool {
        if self.is_disconnected {
            return true;
        }

        if let Ok(_) = self.disconnect.as_mut().unwrap().try_recv() {
            self.is_disconnected = true;
            return true;
        }
        return false;
    }
}
