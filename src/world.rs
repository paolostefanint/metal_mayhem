use super::collisions::{Axis, Body, BodyType, CollisionItem, AABB};
use super::player::Player;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub enum Direction {
    L,
    R,
}

pub struct GameWorld {
    pub current_time: Instant,
    pub size: (f32, f32),
    pub players: Vec<Player>,
}

impl GameWorld {
    pub fn get_players_state(&self) -> Vec<PlayerState> {
        let players = self.players.iter().map(|player| PlayerState {
            id: player.id.clone(),
            p: player.position.clone(),
            dir: Direction::L,
        });
        return players.collect();
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub current_time: f32,
    pub current_state: String,
    pub players: Vec<PlayerState>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    id: u8,
    pub p: (f32, f32),
    pub dir: Direction,
}

impl GameWorld {
    pub fn update(&mut self) {
        let last_time = self.current_time;
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        // println!("delta_time: {}", delta_time
        self.current_time = current_time;

        let mut collision_items: Vec<CollisionItem> = vec![];

        for player in self.players.iter_mut() {
            player.tick(delta_time);
        }

        for player in self.players.iter_mut() {
            let player_aabb = AABB::new(
                (
                    player.position.0 - player.size.0 / 2.0,
                    player.position.1 - player.size.1 / 2.0,
                ),
                (
                    player.position.0 + player.size.0 / 2.0,
                    player.position.1 + player.size.1 / 2.0,
                ),
            );
            player.body.aabb = player_aabb.clone();

            let item = CollisionItem {
                pid: Some(player.id),
                body: player.body.clone(),
            };
            collision_items.push(item);
        }

        let mut collisions: Vec<(&CollisionItem, &CollisionItem, Axis)> = vec![];

        // check collisions
        for i in 0..collision_items.len() {
            for j in i..collision_items.len() {
                let item = &collision_items[i];
                let other = &collision_items[j];
                if item.pid != other.pid {
                    if item.body.aabb.intersects(&other.body.aabb) {
                        let axis = item.body.aabb.get_collision_axis(&other.body.aabb);
                        collisions.push((item, other, axis));
                    }
                }
            }
        }

        if collisions.len() > 0 {
            // println!("collisions: {:?}", collisions);
            for collision in collisions.iter() {
                let mut players_iter = self.players.iter_mut();

                match collision.0.pid {
                    Some(pid) => {
                        let player = players_iter.find(|p| p.id == pid).unwrap();
                        match collision.2 {
                            Axis::X => {
                                if player.position.0 < collision.1.body.aabb.center().0 {
                                    player.position.0 -= 0.1;
                                } else {
                                    player.position.0 += 0.1;
                                }
                            }
                            Axis::Y => {
                                if player.position.1 < collision.1.body.aabb.center().1 {
                                    player.position.1 -= 0.1;
                                } else {
                                    player.position.1 += 0.1;
                                }
                            }
                        }
                    }
                    None => (),
                }

                match collision.1.pid {
                    Some(pid) => {
                        let player = players_iter.find(|p| p.id == pid).unwrap();
                        match collision.2 {
                            Axis::X => {
                                if player.position.0 < collision.0.body.aabb.center().0 {
                                    player.position.0 -= 0.1;
                                } else {
                                    player.position.0 += 0.1;
                                }
                            }
                            Axis::Y => {
                                if player.position.1 < collision.0.body.aabb.center().1 {
                                    player.position.1 -= 0.1;
                                } else {
                                    player.position.1 += 0.1;
                                }
                            }
                        }
                    }
                    None => (),
                }
            }
        }
    }
}
