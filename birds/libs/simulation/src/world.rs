use rand::RngCore;

use crate::{animal::Animal, food::Food};

#[derive(Debug)]
pub struct World {
    _animals: Vec<Animal>,
    _foods: Vec<Food>,
}

impl World {
    pub fn animals(&self) -> &[Animal] {
        &self._animals
    }

    pub fn foods(&self) -> &[Food] {
        &self._foods
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();
        let foods = (0..60).map(|_| Food::random(rng)).collect();

        Self {
            _animals: animals,
            _foods: foods,
        }
    }
}
