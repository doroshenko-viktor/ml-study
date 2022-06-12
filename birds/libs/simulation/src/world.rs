use rand::{Rng, RngCore};

use crate::{animal::Animal, food::Food};

#[derive(Debug)]
pub struct World {
    _animals: Vec<Animal>,
    pub _foods: Vec<Food>,
}

impl World {
    pub fn get_animals_mut(&mut self) -> &mut [Animal] {
        &mut self._animals
    }
    pub fn get_animals(&self) -> &[Animal] {
        &self._animals
    }

    pub fn get_foods(&self) -> &[Food] {
        &self._foods
    }

    pub fn get_foods_mut(&mut self) -> &mut [Food] {
        &mut self._foods
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();
        let foods = (0..60).map(|_| Food::random(rng)).collect();

        Self {
            _animals: animals,
            _foods: foods,
        }
    }

    pub fn process_movements(&mut self) {
        for animal in self._animals.iter_mut() {
            animal.step();
        }
    }

    pub fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in self._animals.iter() {
            for food in self._foods.iter_mut() {
                let distance = nalgebra::distance(animal.position(), food.get_position());

                if distance <= 0.01 {
                    food.set_position(rng.gen());
                }
            }
        }
    }
}
