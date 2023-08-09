use super::world::GameWorld;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn render(world: &GameWorld) {
    // clear screen
    print!("{}[2J", 27 as char);

    for y in 0..world.size.1 as u32 {
        for x in 0..world.size.0 as u32 {
            let mut symbol = String::from("-");

            for p in world.get_players() {
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
