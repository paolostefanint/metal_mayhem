use super::world::GameWorld;
use rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn setup_world(world_size: (f32, f32)) -> GameWorld {
    let mut world = GameWorld {
        current_time: Instant::now(),
        size: world_size,
        players: Vec::new(),
    };
    return world;
}

pub fn start_physics_engine(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
    collider_set: Arc<Mutex<ColliderSet>>,
) -> thread::JoinHandle<()> {
    let physics_thread = thread::spawn(move || {
        println!("physics thread");

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        loop {
            {
                let mut rigid_body_set = match rigid_body_set.lock() {
                    Ok(rigid_body_set) => rigid_body_set,
                    Err(poisoned) => {
                        println!("poisoned rigid_body_set mutex on physics thread");
                        poisoned.into_inner()
                    }
                };
                let mut collider_set = match collider_set.lock() {
                    Ok(collider_set) => collider_set,
                    Err(poisoned) => {
                        println!("poisoned collider_set mutex on physics thread");
                        poisoned.into_inner()
                    }
                };
                physics_pipeline.step(
                    &gravity,
                    &integration_parameters,
                    &mut island_manager,
                    &mut broad_phase,
                    &mut narrow_phase,
                    &mut rigid_body_set,
                    &mut collider_set,
                    &mut impulse_joint_set,
                    &mut multibody_joint_set,
                    &mut ccd_solver,
                    None,
                    &physics_hooks,
                    &event_handler,
                );
            }
            thread::sleep(Duration::from_millis(1000 / 60));
        }
    });
    return physics_thread;
}
