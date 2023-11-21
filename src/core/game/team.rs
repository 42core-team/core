use super::{action::Action, bridge::bridge, State, Game};

use tokio::{net::TcpStream, sync::mpsc::Receiver, sync::mpsc::Sender};

#[allow(dead_code)] // @TODO remove when used
#[derive(Debug)]
pub struct Team {
    pub id: u64,
    pub uuid: String,
    pub name: String,

    pub balance: u64,

	pub sender: Option<Sender<State>>,
	pub receiver: Option<Receiver<Vec<Action>>>,
	pub disconnect: Option<Receiver<()>>, // @TODO disconnect check in the loop 
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
		}
	}

	pub fn get_fake_team() -> Self {
		Team {
			id: 0,
			uuid: String::from("Hello"),
			name: String::from("asdf"),
			balance: 100,
			sender: None,
			receiver: None,
			disconnect: None
		}
	}
}
