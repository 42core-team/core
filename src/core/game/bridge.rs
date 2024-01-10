//!
//! ## Introduction
//! The bridge is used to handel the socket connections.
//!
//!

use std::ops::Add;

use super::{action::{Action, Request}, State};
use serde_json;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

pub(crate) fn bridge(
    stream: TcpStream,
) -> (Sender<State>, Receiver<Vec<Action>>, Receiver<()>) {
    let (mscp_to_socket_sender, mut mscp_to_socket_receiver) = mpsc::channel::<State>(100);
    let (socket_to_mscp_sender, socket_to_mscp_receiver) = mpsc::channel::<Vec<Action>>(100);
    let (disconnect_sender, disconnect_receiver) = mpsc::channel::<()>(1);

    let (mut reader, mut writer) = tokio::io::split(stream);

    tokio::spawn(async move {
        let mut buffer = [0; 2048];
        loop {
            match reader.read(&mut buffer).await {
                Ok(n) if n == 0 => {
                    println!("Connection closed by client");
                    let _ = disconnect_sender.send(()).await;
                    break;
                },
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
                                    // println!("Parsed Actions: {:?}", actions);
                                    let _ = socket_to_mscp_sender.send(actions.actions).await;
                                }
                                Err(err) => {
                                    println!("Parse Error in bridge: {:?}", err);
                                }
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
                Some(state) => {
                    let json_string = serde_json::to_string(&state).unwrap().add("\n");
                    if let Err(_) = writer.write_all(json_string.as_bytes()).await {
                        println!("Send Error in bridge");
                        let _ = writer.shutdown().await;
                        break;
                    }
                }
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
