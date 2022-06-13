mod animal;
mod eye;
mod food;
mod simulation;
mod world;

pub use animal::Animal;
pub use eye::Eye;
pub use food::Food;
pub use simulation::Simulation;
pub use world::World;

use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

