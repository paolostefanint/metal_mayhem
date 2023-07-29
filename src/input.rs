use super::world::GameWorld;
use rand::*;
use rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_input(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
) -> thread::JoinHandle<()> {
    let _fake_input_thread = thread::spawn(move || {
        println!("fake input thread");
        loop {
            {
                let mut world = match world.lock() {
                    Ok(world) => world,
                    Err(poisoned) => {
                        println!("poisoned world mutex on fake input thread");
                        poisoned.into_inner()
                    }
                };
                let mut rigid_body_set = match rigid_body_set.lock() {
                    Ok(rigid_body_set) => rigid_body_set,
                    Err(poisoned) => {
                        println!("poisoned rigid_body_set mutex on fake input thread");
                        poisoned.into_inner()
                    }
                };

                for p in &mut world.players {
                    // get random x impulse between 1 and 10
                    let x_impulse = thread_rng().gen_range(-10.0..10.0);
                    let y_impulse = thread_rng().gen_range(-10.0..10.0);

                    let player_body = &mut rigid_body_set[p.body_handle];
                    player_body.apply_impulse(vector![x_impulse, y_impulse], true);
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });
    return _fake_input_thread;
}
