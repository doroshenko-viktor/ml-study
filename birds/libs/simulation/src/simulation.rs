use crate::{
  animal_individual::AnimalIndividual, world::World, GENERATION_LENGTH,
};
use lib_genetic_algorithm::{
  GaussianMutation, GeneticAlgorithm, RouletteWheelSelection, Statistics,
  UniformCrossover,
};
use rand::RngCore;

pub struct Simulation {
  _world: World,
  _ga: lib_genetic_algorithm::GeneticAlgorithm<RouletteWheelSelection>,
  _age: usize,
}

impl Simulation {
  pub fn world(&self) -> &World {
    &self._world
  }

  pub fn random(rng: &mut dyn RngCore) -> Self {
    let world = World::random(rng);
    let ga = GeneticAlgorithm::new(
      RouletteWheelSelection::default(),
      UniformCrossover::default(),
      GaussianMutation::new(0.01, 0.3),
    );
    Self {
      _world: world,
      _ga: ga,
      _age: 0,
    }
  }

  pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
    self._world.process_movements();
    self._world.process_brains();
    self._world.process_collisions(rng);

    self._age += 1;

    if self._age > GENERATION_LENGTH {
      Some(self.evolve(rng))
    } else {
      None
    }
  }

  fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
    self._age = 0;

    let current_population: Vec<AnimalIndividual> = self
      ._world
      .get_animals()
      .iter()
      .map(AnimalIndividual::from_animal)
      .collect();

    let (evolved_population, stats) = self
      ._ga
      .evolve(rng, &current_population);

    self._world.set_animals(
      evolved_population
        .into_iter()
        .map(|individual| individual.into_animal(rng))
        .collect(),
    );
    self._world.restart_foods(rng);

    stats
  }

  pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
    loop {
      if let Some(summary) = self.step(rng) {
        return summary;
      }
    }
  }
}
