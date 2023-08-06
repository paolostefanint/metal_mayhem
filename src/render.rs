use super::world::GameWorld;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_render(world: Arc<Mutex<GameWorld>>) -> thread::JoinHandle<()> {
    let render_thread = thread::spawn(move || {
        println!("render thread");
        loop {
            // world.handle_inputs(&mut rigid_body_set);
            {
                let world = world.lock().unwrap();
                // render(&world);
                let player = &world.players[0];
                println!("dai player position: {:?}", player.position);
            }
            thread::sleep(Duration::from_millis(10000));
        }
    });
    return render_thread;
}

pub fn render(world: &GameWorld) {
    // clear screen
    print!("{}[2J", 27 as char);

    for y in 0..world.size.1 as u32 {
        for x in 0..world.size.0 as u32 {
            let mut symbol = String::from("-");

            for p in &world.players {
                let player_position = p.position;

                if player_position.0 as u32 == x && player_position.1 as u32 == y {
                    // println!("{:?}", player_position.vector);
                    symbol = p.id.to_string().clone();
                }
            }
            print!(" {}", symbol);
        }
        println!("");
    }
}
