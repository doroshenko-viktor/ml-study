use lib_genetic_algorithm::{Chromosome, Individual};
use rand::RngCore;

use crate::Animal;

pub struct AnimalIndividual {
  _fitness: f32,
  _chromosome: Chromosome,
}

impl Individual for AnimalIndividual {
  fn create(chromosome: Chromosome) -> Self {
    Self {
      _fitness: 0.0,
      _chromosome: chromosome,
    }
  }

  fn chromosome(&self) -> &Chromosome {
    &self._chromosome
  }

  fn fitness(&self) -> f32 {
    self._fitness
  }
}

impl AnimalIndividual {
  pub fn from_animal(animal: &Animal) -> Self {
    Self {
      _fitness: animal.get_satiation() as f32,
      _chromosome: animal.as_chromosome(),
    }
  }

  pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
    Animal::from_chromosome(self._chromosome, rng)
  }
}
