use super::player::Player;
use rapier2d::dynamics::RigidBodySet;
use serde::{Deserialize, Serialize};

pub struct GameWorld {
    pub size: (f32, f32),
    pub players: Vec<Player>,
}

impl GameWorld {
    pub fn get_players_state(&self, rigid_body_set: &RigidBodySet) -> Vec<PlayerState> {
        let players = self.players.iter().map(|player| {
            let rigid_body = &rigid_body_set[player.body_handle];
            PlayerState {
                id: player.id.clone(),
                p: (
                    rigid_body.position().translation.x,
                    rigid_body.position().translation.y,
                ),
                dir: (
                    rigid_body.position().rotation.re.cos(),
                    rigid_body.position().rotation.re.sin(),
                ),
            }
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
    pub dir: (f32, f32),
}

impl GameWorld {}
