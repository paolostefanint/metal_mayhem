use super::collisions::{Axis, Body, BodyType, CollisionItem, AABB};
use super::player::{EntityType, Player};

use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub enum Direction {
    L,
    R,
}

pub struct GameWorld {
    pub current_time: Instant,
    pub size: (f32, f32),
    pub entities: HashMap<u32, Box<dyn GameEntity + Send + 'static>>,
}

pub trait GameEntity {
    fn get_id(&self) -> u32;
    fn get_body(&self) -> Body;
    fn get_entity_type(&self) -> EntityType;
    fn tick(&mut self, delta_time: f32);
    fn as_player_mut(&mut self) -> &mut Player;
    fn as_player(&self) -> &Player;
}

impl GameWorld {
    pub fn new(world_size: (f32, f32)) -> GameWorld {
        let mut world = GameWorld {
            current_time: Instant::now(),
            size: world_size,
            entities: HashMap::new(),
        };
        return world;
    }
    pub fn get_players(&self) -> Vec<&Player> {
        let players = self
            .entities
            .values()
            .filter_map(|entity| match entity.get_entity_type() {
                EntityType::Player => Some(entity),
                _ => None,
            })
            .map(|entity| {
                let player = entity.as_player();
                return player;
            })
            .collect::<Vec<&Player>>();

        return players;
    }
    pub fn get_players_mut(&mut self) -> Vec<&mut Player> {
        let players = self
            .entities
            .values_mut()
            .filter_map(|entity| match entity.get_entity_type() {
                EntityType::Player => Some(entity),
                _ => None,
            })
            .map(|entity| {
                let player = entity.as_player_mut();
                return player;
            })
            .collect::<Vec<&mut Player>>();

        return players;
    }
    pub fn get_players_state(&self) -> Vec<PlayerState> {
        let players = self.get_players();
        let players = players.iter().map(|player| PlayerState {
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
    id: u32,
    pub p: (f32, f32),
    pub dir: Direction,
}

impl GameWorld {
    pub fn add_entity(&mut self, entity: Box<dyn GameEntity + Send + 'static>) {
        self.entities.insert(entity.get_id(), entity);
    }
    pub fn update(&mut self) {
        let last_time = self.current_time;
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        // println!("delta_time: {}", delta_time
        self.current_time = current_time;

        // update player position and body
        // for player in self.players.iter_mut() {
        //     player.tick(delta_time);
        // }

        for entity in self.entities.values_mut() {
            entity.tick(delta_time);
        }

        // get all collisions

        // if entity collides with other_entity
        // entity.handle_collision(other_entity);

        // handle collisions
        //
        // elements_colliding.handle_collision(other_element);

        // update player aabb
        // let mut collisions: Vec<(&CollisionItem, &CollisionItem, Axis)> = vec![];

        // check collisions
        // for i in 0..collision_items.len() {
        //     for j in i..collision_items.len() {
        //         let item = &collision_items[i];
        //         let other = &collision_items[j];
        //         if item.pid != other.pid {
        //             if item.body.aabb.intersects(&other.body.aabb) {
        //                 let axis = item.body.aabb.get_collision_axis(&other.body.aabb);

        //                 println!("axis: {:?}", axis);

        //                 // collisions.push((item, other, axis));
        //             }
        //         }
        //     }
        // }

        // if collisions.len() > 0 {
        //     // println!("collisions: {:?}", collisions);
        //     for collision in collisions.iter() {
        //         let mut players_iter = self.players.iter_mut();

        //         match collision.0.pid {
        //             Some(pid) => {
        //                 let player = players_iter.find(|p| p.id == pid).unwrap();
        //                 match collision.2 {
        //                     Axis::X => {
        //                         if player.position.0 < collision.1.body.aabb.center().0 {
        //                             player.position.0 -= 0.1;
        //                         } else {
        //                             player.position.0 += 0.1;
        //                         }
        //                     }
        //                     Axis::Y => {
        //                         if player.position.1 < collision.1.body.aabb.center().1 {
        //                             player.position.1 -= 0.1;
        //                         } else {
        //                             player.position.1 += 0.1;
        //                         }
        //                     }
        //                 }
        //             }
        //             None => (),
        //         }

        //         match collision.1.pid {
        //             Some(pid) => {
        //                 let player = players_iter.find(|p| p.id == pid).unwrap();
        //                 match collision.2 {
        //                     Axis::X => {
        //                         if player.position.0 < collision.0.body.aabb.center().0 {
        //                             player.position.0 -= 0.1;
        //                         } else {
        //                             player.position.0 += 0.1;
        //                         }
        //                     }
        //                     Axis::Y => {
        //                         if player.position.1 < collision.0.body.aabb.center().1 {
        //                             player.position.1 -= 0.1;
        //                         } else {
        //                             player.position.1 += 0.1;
        //                         }
        //                     }
        //                 }
        //             }
        //             None => (),
        //         }
        //     }
        // }
    }
}
