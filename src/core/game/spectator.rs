use tokio::{
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use super::{bridge::bridge, Message, Team};
#[derive(Debug)]

pub struct Spectator {
    pub sender: Option<Sender<Message>>,
    pub receiver: Option<Receiver<Message>>,
    pub disconnect: Option<Receiver<()>>,
    pub is_disconnected: bool,
}

impl Spectator {
    pub fn new(stream: TcpStream) -> Self {
        let (sender, receiver, disconnect) = bridge(stream);
        Self {
            sender: Some(sender),
            receiver: Some(receiver),
            disconnect: Some(disconnect),
            is_disconnected: false,
        }
    }

    pub fn from_team(team: Team) -> Self {
        Self {
            sender: team.sender,
            receiver: team.receiver,
            disconnect: team.disconnect,
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
