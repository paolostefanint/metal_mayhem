use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const WORLD_SIZE: (f32, f32) = (20.0, 20.0);

struct GameWorld {
    size: (f32, f32),
}

impl GameWorld {
    fn position_in_world(&self, position: (f32, f32)) -> bool {
        position.0 >= 0.0
            && position.0 <= self.size.0
            && position.1 >= 0.0
            && position.1 <= self.size.1
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

// derive debug
#[derive(Debug)]
struct Player {
    id: u8,
    attack: f32,
    defense: f32,
    base_movement: f32,
    direction: Direction,
    health: f32,
    position: (f32, f32),
    size: f32,
    input: PlayerInputs,
}

#[derive(Debug, Clone, Copy)]
struct PlayerInputs {
    north: bool,
    north_east: bool,
    east: bool,
    south_east: bool,
    south: bool,
    south_west: bool,
    west: bool,
    north_west: bool,
}

impl PlayerInputs {
    fn new() -> PlayerInputs {
        PlayerInputs {
            north: false,
            north_east: false,
            east: false,
            south_east: false,
            south: false,
            south_west: false,
            west: false,
            north_west: false,
        }
    }
    fn reset(&mut self) {
        self.north = false;
        self.north_east = false;
        self.east = false;
        self.south_east = false;
        self.south = false;
        self.south_west = false;
        self.west = false;
        self.north_west = false;
    }
}

impl Player {
    fn tick(&mut self, delta_time: time::Duration) -> (u8, (f32, f32), Direction) {
        let delta_seconds = delta_time.as_secs_f32();

        let mut new_position = (self.position.0, self.position.1);
        let mut direction = self.direction;

        if self.input.north {
            new_position.1 += self.base_movement * delta_seconds;
            direction = Direction::North;
        }
        if self.input.north_east {
            let angle = 45.0f32.to_radians();
            new_position.0 += self.base_movement * delta_seconds * angle.cos();
            new_position.1 += self.base_movement * delta_seconds * angle.cos();
            direction = Direction::NorthEast;
        }
        if self.input.south {
            new_position.1 -= self.base_movement * delta_seconds;
            direction = Direction::South;
        }
        if self.input.west {
            new_position.0 -= self.base_movement * delta_seconds;
            direction = Direction::West;
        }
        if self.input.east {
            new_position.0 += self.base_movement * delta_seconds;
            direction = Direction::East;
        }

        if new_position.0 < 0.0 {
            new_position.0 = self.position.0;
        }
        if new_position.0 > WORLD_SIZE.0 {
            new_position.0 = self.position.0;
        }
        if new_position.1 < 0.0 {
            new_position.1 = self.position.1;
        }
        if new_position.1 > WORLD_SIZE.1 {
            new_position.1 = self.position.1;
        }

        return (self.id, new_position, direction);
    }
    fn collides_with(&self, other: &Player) -> bool {
        let distance = (self.position.0 - other.position.0).powi(2)
            + (self.position.1 - other.position.1).powi(2);
        let radius = self.size + other.size;
        distance < radius.powi(2)
    }
    fn is_in_world(&self, world: &GameWorld) -> bool {
        self.position.0 >= 0.0
            && self.position.0 <= world.size.0
            && self.position.1 >= 0.0
            && self.position.1 <= world.size.1
    }
}

struct GameState {
    iteration: u32,
    world: GameWorld,
    players: Vec<Player>,
}

impl GameState {
    fn tick(&mut self, delta_time: time::Duration, keyboard_input: &PlayerInputs) {
        self.handle_input(keyboard_input);

        for player in &mut self.players {
            let (_id, new_position, new_direction) = player.tick(delta_time);
            player.position = new_position;
            player.direction = new_direction;
        }
    }
    fn handle_input(&mut self, keyboard_input: &PlayerInputs) {
        for player in &mut self.players {
            if player.id == 1 {
                if keyboard_input.up {
                    player.input.up = true;
                }
                if keyboard_input.down {
                    player.input.down = true;
                }
                if keyboard_input.left {
                    player.input.left = true;
                }
                if keyboard_input.right {
                    player.input.right = true;
                }
            }
        }
    }
    fn reset_input(&mut self) {
        for player in &mut self.players {
            player.input = PlayerInputs::new();
        }
    }
    fn check_collisions(&mut self) {
        // return a list of collision objects
        let mut collisions: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.players.len() {
            for j in i + 1..self.players.len() {
                if self.players[i].collides_with(&self.players[j]) {
                    collisions.push((i, j));
                }
            }
        }
    }
}

fn main() {
    let player1 = Player {
        id: 1,
        attack: 10.0,
        defense: 10.0,
        base_movement: 15.0,
        direction: Direction::North,
        health: 100.0,
        size: 10.0,
        position: (0.0, 0.0),
        input: PlayerInputs::new(),
    };

    let player2 = Player {
        id: 2,
        attack: 10.0,
        defense: 10.0,
        base_movement: 5.0,
        direction: Direction::North,
        health: 100.0,
        size: 10.0,
        position: (10.0, 10.0),
        input: PlayerInputs::new(),
    };

    let mut game_state = GameState {
        players: vec![player1, player2],
        iteration: 0,
        world: GameWorld { size: WORLD_SIZE },
    };

    let target_fps = 10;
    let target_delta_time = time::Duration::from_millis(1000 / target_fps);
    let mut last_tick = time::Instant::now();
    let start_time = last_tick;

    println!("Starting game loop, target fps: {}", target_fps);
    println!("Target delta time: {:?}", target_delta_time);

    let keyboard_input = PlayerInputs::new();
    let keyboard_input_mutex = Mutex::new(keyboard_input);
    let keyboard_input_arc = Arc::new(keyboard_input_mutex);

    // FAKE INPUT THREAD
    let keyboard_input_arc_clone = Arc::clone(&keyboard_input_arc);
    let input_thread = thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("input thread");
        let mut input = keyboard_input_arc_clone.lock().unwrap();

        // get a random direction and apply it to the input
        let mut rng = rand::thread_rng();
        let random_number: f32 = rng.gen();
        if random_number < 0.125 {
            input.down = true;
        } else if random_number < 0.25 {
            input.up = true;
        } else if random_number < 0.75 {
            input.left = true;
        } else {
            input.right = true;
        }

        input.up = true;
    });

    loop {
        let current_frame_time = time::Instant::now();
        let delta_time = current_frame_time - last_tick;
        last_tick = current_frame_time;

        // println!("delta time: {:?}", delta_time);

        // keep this scope as it is ESSENTIAL to unlock the mutex
        {
            let keyboard_input = Arc::clone(&keyboard_input_arc);
            let mut keyboard_input = keyboard_input.lock().unwrap();
            // println!("{:?}", &keyboard_input);

            game_state.tick(delta_time, &keyboard_input);

            keyboard_input.reset();
        }

        render(&game_state);

        let elapsed_time = current_frame_time.elapsed();
        // println!("elapsed time: {:?}", elapsed_time);
        if elapsed_time < target_delta_time {
            let sleep_interval = target_delta_time - elapsed_time;
            println!("sleep interval: {:?}", sleep_interval);
            thread::sleep(sleep_interval);
        }

        println!("total elapsed time: {:?}", start_time.elapsed());
    }
}

fn render(game_state: &GameState) {
    // render the game state
    // println!("***** iteration {} *******", game_state.iteration);
    // println!("players: {:?}", game_state.players);
    // println!("***** end iteration {} *******", game_state.iteration);

    print!("{}[2J", 27 as char);
    for y in 0..game_state.world.size.1 as u32 {
        for x in 0..game_state.world.size.0 as u32 {
            let mut symbol = String::from("-");

            for p in &game_state.players {
                if p.position.0 as u32 == x && p.position.1 as u32 == y {
                    symbol = p.id.to_string().clone();
                }
            }
            print!(" {}", symbol);
        }
        println!("");
    }
}
