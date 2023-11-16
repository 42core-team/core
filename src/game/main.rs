use core::game::{Game, Team};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let mut queue: Vec<Team> = Vec::<Team>::new();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        queue.push(Team::from_tcpStream(stream));

        if queue.len() >= 2 {
            let t1 = queue.remove(0);
            let t2 = queue.remove(0);
            let mut game = Game::new(vec![t1, t2]);

            tokio::spawn(async move {
                println!("Game start!");
                game.start().await;
                println!("Game ended!");
            });
        }
    }
}
