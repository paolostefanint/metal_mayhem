use super::world::GameWorld;
use futures_util::SinkExt;
use rand::*;
use rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_websockets::{Error, Message, ServerBuilder};

pub async fn start_input(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
) -> Result<(), Error> {
    println!("Start input thread begin");
    let listener = TcpListener::bind("127.0.0.1:40020").await?;

    tokio::spawn(async move {
        println!("Start input thread");
        while let Ok((stream, _)) = listener.accept().await {
            let mut ws_stream = ServerBuilder::new().accept(stream).await?;

            tokio::spawn(async move {
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(msg) => {
                            println!("Received a message from a client: ");
                            // println!("Received a message from a client: {}", msg);
                            // ws_stream.send(Message::text("Hello from server")).await?;
                        }
                        Err(e) => {
                            println!("Error receiving message: {}", e);
                        }
                    }
                }
            });
        }
        Ok::<_, Error>(())
    });
    Ok::<_, Error>(())
    // let _fake_input_thread = thread::spawn(move || {
    //     println!("fake input thread");
    //     loop {
    //         {
    //             let mut world = match world.lock() {
    //                 Ok(world) => world,
    //                 Err(poisoned) => {
    //                     println!("poisoned world mutex on fake input thread");
    //                     poisoned.into_inner()
    //                 }
    //             };
    //             let mut rigid_body_set = match rigid_body_set.lock() {
    //                 Ok(rigid_body_set) => rigid_body_set,
    //                 Err(poisoned) => {
    //                     println!("poisoned rigid_body_set mutex on fake input thread");
    //                     poisoned.into_inner()
    //                 }
    //             };

    //             for p in &mut world.players {
    //                 // get random x impulse between 1 and 10
    //                 let x_impulse = thread_rng().gen_range(-10.0..10.0);
    //                 let y_impulse = thread_rng().gen_range(-10.0..10.0);

    //                 let player_body = &mut rigid_body_set[p.body_handle];
    //                 player_body.apply_impulse(vector![x_impulse, y_impulse], true);
    //             }
    //         }
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // });
    // return _fake_input_thread;
}
