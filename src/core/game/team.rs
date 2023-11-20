use super::{action::Action, bridge::bridge, State, Game};

use tokio::{net::TcpStream, sync::mpsc::Receiver, sync::mpsc::Sender};

#[allow(dead_code)] // @TODO remove when used
#[derive(Debug)]
pub struct Team {
    pub id: u64,
    pub uuid: String,
    pub name: String,

    pub balance: u64,

    pub sender: Sender<State>,
    pub receiver: Receiver<Vec<Action>>,
    pub disconnect: Receiver<()>, // @TODO disconnect check in the loop 
}

impl Team {
    pub fn from_tcp_stream(stream: TcpStream) -> Self {
        let (sender, receiver, disconnect) = bridge(stream);

        Team {
            id: Game::generate_u64_id(),
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
    pub fn update(team_id: u64, receiver: &mut Receiver<Vec<Action>>, game: &mut Game) {
        while let Ok(actions) = receiver.try_recv() {
            println!("TEAM send action: {:?}", actions);
			for action in actions {
				match action {
					Action::Create(action) => {
						println!("Create unit of type {:?}", action);
						Game::create_unit(game, team_id, action.type_id);
					},
					Action::Travel(action) => {
						println!("Travel unit of type {:?}", action);
						// @TODO handel travel action here
					},
					Action::Attack(action) => {
						println!("Attack unit of type {:?}", action);
						// @TODO handel attack action here
						// changed line
					}
				}
			}
        }
    }
}
