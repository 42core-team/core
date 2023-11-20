use super::{action::Action, bridge::bridge, State};

use tokio::{net::TcpStream, sync::mpsc::Receiver, sync::mpsc::Sender};

#[allow(dead_code)] // @TODO remove when used
pub struct Team {
    pub id: u64,
    uuid: String,
    name: String,

    pub balance: u64,

    pub sender: Sender<State>,
    receiver: Receiver<Vec<Action>>,
    disconnect: Receiver<()>, // @TODO disconnect check in the loop 
}

impl Team {
    pub fn from_tcp_stream(stream: TcpStream) -> Self {
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

	///
	/// Function to handel the actions received from the client
	/// 
	/// a valid json to send with netcat is:
	/// [{"Create":{"type_id":3}},{"Travel":{"id":1,"x":2,"y":3}},{"Attack":{"attacker_id":1,"target_id":2}}]
	/// 
	/// To uns netcat:
	/// ```sh
	/// nc localhost 4242
	/// ```
	/// then paste the json and press enter
	/// 
	/// You need at least two netcat instances to start a game
	/// 
    pub fn update(&mut self) {
        while let Ok(actions) = self.receiver.try_recv() {
            println!("TEAM {:?} send action: {:?}", self.name, actions);
			for action in actions {
				match action {
					Action::Create(action) => {
						println!("Create unit of type {:?}", action);
						// @TODO handel create action here
					},
					Action::Travel(action) => {
						println!("Travel unit of type {:?}", action);
						// @TODO handel travel action here
					},
					Action::Attack(action) => {
						println!("Attack unit of type {:?}", action);
						// @TODO handel attack action here
						// Attack code added
					}
				}
			}
        }
		match self.disconnect.try_recv() {
			Ok(()) => {
				println!("Disconnect signal received");
				// @TODO Handle disconnect here
			},
			Err(_) => {
				// No disconnect signal received 
			}
		}
    }
}
