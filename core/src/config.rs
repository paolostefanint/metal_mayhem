use configparser::ini::Ini;
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct TiziaConfig {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub health: f32,
}

#[derive(Debug)]
pub struct GundamConfig {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub health: f32,
}

#[derive(Debug)]
pub struct CosoConfig {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub health: f32,
}

#[derive(Debug)]
pub struct HitboxConfig {
    pub diff_x: f32,
    pub diff_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

#[derive(Debug)]
pub struct Config {
    pub world_size: (f32, f32),
    pub round_duration: u64,
    pub player_size: (f32, f32),
    pub damage_duration: f32,
    pub hitbox: HitboxConfig,
    pub tizia: TiziaConfig,
    pub gundam: GundamConfig,
    pub coso: CosoConfig,
}
pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init_config() -> () {
    let mut config = Ini::new();

    println!("Loading config");
    println!("Current dir: {:?}", std::env::current_dir().unwrap());

    let config_path = std::env::current_dir().unwrap().join("props.ini");
    config.load(config_path).unwrap();

    let world_size_x = config.getfloat("world", "size_x").unwrap().unwrap() as f32;
    let world_size_y = config.getint("world", "size_y").unwrap().unwrap() as f32;
    let round_duration = config.getint("game", "round_duration").unwrap().unwrap() as u64;

    let program_config = Config {
        world_size: (world_size_x, world_size_y),
        round_duration,
        damage_duration: config
            .getfloat("player", "damage_duration")
            .unwrap()
            .unwrap() as f32,
        player_size: (
            config.getfloat("player", "size_x").unwrap().unwrap() as f32,
            config.getfloat("player", "size_y").unwrap().unwrap() as f32,
        ),
        hitbox: HitboxConfig {
            diff_x: config.getfloat("hitbox", "diff_x").unwrap().unwrap() as f32,
            diff_y: config.getfloat("hitbox", "diff_y").unwrap().unwrap() as f32,
            size_x: config.getfloat("hitbox", "size_x").unwrap().unwrap() as f32,
            size_y: config.getfloat("hitbox", "size_y").unwrap().unwrap() as f32,
        },
        tizia: TiziaConfig {
            attack: config.getfloat("tizia", "attack").unwrap().unwrap() as f32,
            defense: config.getfloat("tizia", "defense").unwrap().unwrap() as f32,
            speed: config.getfloat("tizia", "speed").unwrap().unwrap() as f32,
            health: config.getfloat("tizia", "health").unwrap().unwrap() as f32,
        },
        gundam: GundamConfig {
            attack: config.getfloat("gundam", "attack").unwrap().unwrap() as f32,
            defense: config.getfloat("gundam", "defense").unwrap().unwrap() as f32,
            speed: config.getfloat("gundam", "speed").unwrap().unwrap() as f32,
            health: config.getfloat("gundam", "health").unwrap().unwrap() as f32,
        },
        coso: CosoConfig {
            attack: config.getfloat("coso", "attack").unwrap().unwrap() as f32,
            defense: config.getfloat("coso", "defense").unwrap().unwrap() as f32,
            speed: config.getfloat("coso", "speed").unwrap().unwrap() as f32,
            health: config.getfloat("coso", "health").unwrap().unwrap() as f32,
        },
    };

    CONFIG.set(program_config).unwrap();
}
