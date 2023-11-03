extern crate core;

use core::socket_server::start_server;

fn main() {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let _ = rt.block_on(start_server());
	loop {
		println!("HELLO game");
	}
}
