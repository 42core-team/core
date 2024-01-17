use lib::game::{Game, Team};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let mut queue: Vec<Team> = Vec::<Team>::new();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        queue.push(Team::from_tcp_stream(stream));

        if queue.len() >= 2 {
            let t1 = queue.remove(0);
            let t2 = queue.remove(0);
            let mut game = Game::new(vec![t1, t2]);

            tokio::spawn(async move {
                log(LogOptions::Info, "Game start!");
                game.start().await;
                log(LogOptions::Info, "Game ended!");
            });
        }
    }
}
