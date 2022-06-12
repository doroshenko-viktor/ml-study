use rand::RngCore;

use crate::world::World;

pub struct Simulation {
    _world: World,
}

impl Simulation {
    pub fn world(&self) -> &World {
        &self._world
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            _world: World::random(rng),
        }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self._world.process_movements();
        self._world.process_collisions(rng);
    }
}
