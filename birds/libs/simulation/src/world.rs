use nalgebra::Rotation2;
use rand::{Rng, RngCore};

use crate::{animal::Animal, food::Food};
use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;

const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

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
        let animals = (0..60).map(|_| Animal::random(rng)).collect();
        let foods = (0..60).map(|_| Food::random(rng)).collect();

        Self {
            _animals: animals,
            _foods: foods,
        }
    }

    pub fn process_brains(&mut self) {
        for animal in &mut self._animals.iter_mut() {
            let vision = animal.process_vision(&self._foods);
            let response = animal.propagate(vision);

            // let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            // let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // animal.set_speed((animal.get_speed() + speed).clamp(SPEED_MIN, SPEED_MAX));
            // animal.set_rotation(Rotation2::new(animal.get_angle() + rotation));
        }
    }

    pub fn process_vision(&mut self, animal: &mut Animal) -> Vec<f32> {
        let vision = animal.process_vision(&self._foods);
        let response = animal.propagate(vision);
        return response;
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
