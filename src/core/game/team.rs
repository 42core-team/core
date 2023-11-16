use super::{action::Action, bridge::bridge, State};

use tokio::{net::TcpStream, sync::mpsc::Receiver, sync::mpsc::Sender};

pub struct Team {
    pub id: u64,
    uuid: String,
    name: String,

    pub balance: u64,

    pub sender: Sender<State>,
    receiver: Receiver<Vec<Action>>,
    disconnect: Receiver<()>,
}

impl Team {
    pub fn from_tcpStream(stream: TcpStream) -> Self {
        let (sender, receiver, disconnect) = bridge(stream);

        Team {
            id: 1212,
            uuid: String::from("Hello"),
            name: String::from("asdf"),
            balance: 100,
            sender,
            receiver,
            disconnect,
        }
    }

    pub fn update(&mut self) {
        while let Ok(actions) = self.receiver.try_recv() {
            println!("TEAM action: {:?}", actions);
        }
    }
}
