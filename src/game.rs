use crate::player::Player;
use crate::world::GameWorld;

pub enum GamePhases {
    WaitingForPlayers,
    RoundCountdown,
    Running,
    RoundEnd,
}

pub struct Game {
    phase: GamePhases,
    world: GameWorld,
}

impl Game {
    pub fn new(world_size: (f32, f32)) -> Game {
        Game {
            phase: GamePhases::WaitingForPlayers,
            world: GameWorld::new(world_size),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.world.add_entity(Box::new(player));
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn get_world(&self) -> &GameWorld {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut GameWorld {
        &mut self.world
    }
}
