use super::{bridge::bridge, Game, Message};
use tokio::{net::TcpStream, sync::mpsc::Receiver, sync::mpsc::Sender};

#[derive(Debug)]
pub struct Team {
    pub id: u64,
    pub uuid: String,
    pub name: String,

    pub balance: u64,

    pub sender: Option<Sender<Message>>,
    pub receiver: Option<Receiver<Message>>,
    pub disconnect: Option<Receiver<()>>,
    pub is_disconnected: bool,
}

impl Team {
    pub fn from_tcp_stream(stream: TcpStream) -> Self {
        let (sender, receiver, disconnect) = bridge(stream);

        Team {
            id: Game::generate_u64_id(),
            uuid: String::from("Hello"),
            name: String::from("asdf"),
            balance: 100,
            sender: Some(sender),
            receiver: Some(receiver),
            disconnect: Some(disconnect),
            is_disconnected: false,
        }
    }

    pub fn get_fake_team(id: u64, name: String) -> Self {
        Team {
            id: id,
            uuid: String::from("Hello"),
            name: name,
            balance: 100,
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
        // @TODO commented out for testing
        // if let None = self.disconnect {
        // 	return true;
        // }

        if let Ok(_) = self.disconnect.as_mut().unwrap().try_recv() {
            self.is_disconnected = true;
            return true;
        }
        return false;
    }
}
