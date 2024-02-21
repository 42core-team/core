//!
//! ## Introduction
//! The bridge is used to handel the socket connections.
//!
//!

use super::{action::Request, config::GameConfigWithId, log::log, Login, Message, State};
use serde_json;
use std::ops::Add;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

pub fn bridge(stream: TcpStream) -> (Sender<Message>, Receiver<Message>, Receiver<()>) {
    let (mscp_to_socket_sender, mut mscp_to_socket_receiver) = mpsc::channel::<Message>(100);
    let (socket_to_mscp_sender, socket_to_mscp_receiver) = mpsc::channel::<Message>(100);
    let (disconnect_sender, disconnect_receiver) = mpsc::channel::<()>(1);

    let (mut reader, mut writer) = tokio::io::split(stream);

    tokio::spawn(async move {
        let mut buffer = [0; 2048];
        loop {
            match reader.read(&mut buffer).await {
                Ok(n) if n == 0 => {
                    log::info("Connection closed by client");
                    let _ = disconnect_sender.send(()).await;
                    break;
                }
                Ok(n) => {
                    if let Ok(s) = std::str::from_utf8(&buffer[..n]) {
                        let msg = s.to_string();
                        let lines: Vec<&str> = msg.split('\n').collect();
                        for line in lines {
                            // println!("LINE {:?}", line);
                            if line.len() == 0 {
                                continue;
                            }
                            match convert_to_actions(line) {
                                Ok(actions) => {
                                    let _ = socket_to_mscp_sender
                                        .send(Message::from_vec_action(actions.actions))
                                        .await;
                                }
                                // at the first send the game config gets send
                                Err(_) => match convert_to_config(line) {
                                    Ok(config) => {
                                        let _ = socket_to_mscp_sender
                                            .send(Message::from_game_config(&config))
                                            .await;
                                    }
                                    Err(_) => match convert_to_state(line) {
                                        Ok(state) => {
                                            let _ = socket_to_mscp_sender
                                                .send(Message::from_state(&state))
                                                .await;
                                        }
                                        Err(_) => match convert_to_login(line) {
                                            Ok(login) => {
                                                let _ = socket_to_mscp_sender
                                                    .send(Message::from_login(&login))
                                                    .await;
                                            }
                                            Err(err) => {
                                                let _ = socket_to_mscp_sender
                                                    .send(Message::from_vec_action(vec![]))
                                                    .await;
                                                log::error(&format!(
                                                    "Parse Error in bridge: {:?} from {:?}",
                                                    err, line
                                                ));
                                            }
                                        },
                                    },
                                },
                            }
                        }
                    };
                }
                Err(_e) => {
                    let _ = disconnect_sender.send(()).await;
                    break;
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            match mscp_to_socket_receiver.recv().await {
                Some(message) => match message {
                    Message::State(state) => {
                        let json_string = serde_json::to_string(&state).unwrap().add("\n");
                        if let Err(_) = writer.write_all(json_string.as_bytes()).await {
                            log::error("Send State Error in bridge");
                            let _ = writer.shutdown().await;
                            break;
                        }
                    }
                    Message::GameConfigWithId(game_config) => {
                        let json_string = serde_json::to_string(&game_config).unwrap().add("\n");
                        if let Err(_) = writer.write_all(json_string.as_bytes()).await {
                            log::error("Send Config Error in bridge");
                            let _ = writer.shutdown().await;
                            break;
                        }
                    }
                    Message::VecAction(vec_action) => {
                        let json_string = serde_json::to_string(&vec_action).unwrap().add("\n");
                        let _ = writer.write_all(json_string.as_bytes()).await;
                        let _ = writer.flush().await;
                    }
                    Message::Login(login) => {
                        let json_string = serde_json::to_string(&login).unwrap().add("\n");
                        let _ = writer.write_all(json_string.as_bytes()).await;
                        let _ = writer.flush().await;
                    }
                },
                None => {
                    break;
                }
            }
        }
    });

    // tokio::spawn(async move {
    //     loop {
    //         match mscp_to_socket_receiver.recv().await {
    //             Some(msg) => {
    //                 let _ = writer.write_all("\n")).await;
    //                 let _ = writer.flush().await;
    //             }
    //             None => {
    //                 break;
    //             }
    //         }
    //     }
    // });

    return (
        mscp_to_socket_sender,
        socket_to_mscp_receiver,
        disconnect_receiver,
    );
}

fn convert_to_actions(buffer: &str) -> Result<Request, serde_json::Error> {
    let result: Result<Request, serde_json::Error> = serde_json::from_str(&buffer);
    result
}

fn convert_to_config(buffer: &str) -> Result<GameConfigWithId, serde_json::Error> {
    let result: Result<GameConfigWithId, serde_json::Error> = serde_json::from_str(&buffer);
    result
}

fn convert_to_state(buffer: &str) -> Result<State, serde_json::Error> {
    let result: Result<State, serde_json::Error> = serde_json::from_str(&buffer);
    result
}

fn convert_to_login(buffer: &str) -> Result<Login, serde_json::Error> {
    let result: Result<Login, serde_json::Error> = serde_json::from_str(&buffer);
    result
}

// in case the serde json parser is not able to parse the json string
//
// fn remove_after_last_brace(input: &str) -> String {
//     let reversed_string: String = input.chars().rev().collect();

//     if let Some(index) = reversed_string.find(']') {
//         let truncated_string = &reversed_string[index..].chars().rev().collect::<String>();
//         truncated_string.to_string()
//     } else {
//         input.to_string()
//     }
// }
