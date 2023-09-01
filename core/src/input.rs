use super::ServerCommand;
use super::ServerCommandMessage;
use futures_util::SinkExt;
use std::sync::mpsc::Sender;
use tokio::net::TcpListener;
use tokio_websockets::{Error, ServerBuilder};

const COMMAND_SPLIT_TOKEN: &str = "|||";
pub async fn start_server_listening_websocket(
    tx: Sender<ServerCommandMessage>,
) -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:40010").await?;

    tokio::spawn(async move {
        println!("Start server listener...");
        while let Ok((stream, _)) = listener.accept().await {
            println!(
                "Server listener accepted a connection from {:?}",
                stream.peer_addr().unwrap()
            );
            let mut ws_stream = ServerBuilder::new().accept(stream).await?;
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(msg) => {
                        // println!("Received a message from the server: {:?}", msg);

                        let text_message = match msg.as_text() {
                            Ok(text) => text,
                            Err(e) => {
                                println!("Error parsing message: {}", e);
                                continue;
                            }
                        };

                        let text_tokens = text_message
                            .split(COMMAND_SPLIT_TOKEN)
                            .collect::<Vec<&str>>();

                        match text_tokens.get(0) {
                            Some(&"start") => {
                                println!("Start game");
                                let data = match text_tokens.get(1) {
                                    Some(&data) => data,
                                    None => "",
                                };
                                tx.send(ServerCommandMessage {
                                    command: ServerCommand::Start,
                                    data: String::from(data),
                                })
                                .unwrap();
                            }
                            Some(&"stop") => {
                                println!("Stop game");
                                let data = match text_tokens.get(1) {
                                    Some(&data) => data,
                                    None => "",
                                };
                                tx.send(ServerCommandMessage {
                                    command: ServerCommand::Stop,
                                    data: String::from(data),
                                })
                                .unwrap();
                            }
                            Some(&"input") => {
                                let data = match text_tokens.get(1) {
                                    Some(&data) => data,
                                    None => "",
                                };
                                tx.send(ServerCommandMessage {
                                    command: ServerCommand::PlayerInput,
                                    data: String::from(data),
                                })
                                .unwrap();
                            }
                            _ => {
                                println!("Unknown command");
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error receiving message: {}", e);
                    }
                }
            }
        }
        Ok::<(), Error>(())
    });
    Ok::<(), Error>(())
}
