use super::world::GameWorld;
use rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_render(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
) -> thread::JoinHandle<()> {
    let render_thread = thread::spawn(move || {
        println!("render thread");
        loop {
            // world.handle_inputs(&mut rigid_body_set);
            {
                let world = match world.lock() {
                    Ok(world) => world,
                    Err(poisoned) => {
                        println!("poisoned world mutex on render thread");
                        poisoned.into_inner()
                    }
                };
                let rigid_body_set = match rigid_body_set.lock() {
                    Ok(rigid_body_set) => rigid_body_set,
                    Err(poisoned) => {
                        println!("poisoned rigid_body_set mutex on render thread");
                        poisoned.into_inner()
                    }
                };
                render(&world, &rigid_body_set);
            }
            thread::sleep(Duration::from_millis(200));
        }
    });
    return render_thread;
}

fn render(world: &GameWorld, rigid_body_set: &RigidBodySet) {
    // clear screen
    print!("{}[2J", 27 as char);

    for y in 0..world.size.1 as u32 {
        for x in 0..world.size.0 as u32 {
            let mut symbol = String::from("-");

            for p in &world.players {
                let player = &rigid_body_set[p.body_handle];
                let player_position = player.position().translation;

                if player_position.x as u32 == x && player_position.y as u32 == y {
                    // println!("{:?}", player_position.vector);
                    symbol = p.id.to_string().clone();
                }
            }
            print!(" {}", symbol);
        }
        println!("");
    }
}
