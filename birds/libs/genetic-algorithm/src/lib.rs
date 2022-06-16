mod chromosome;
mod gaussian_mutation;
mod genetic_algoritym;
mod roulette_wheel_selection;
mod uniform_crossover;

pub use chromosome::Chromosome;
pub use gaussian_mutation::GaussianMutation;
pub use genetic_algoritym::GeneticAlgorithm;
use rand::RngCore;
pub use roulette_wheel_selection::RouletteWheelSelection;
pub use uniform_crossover::UniformCrossover;
pub trait Individual {
  fn fitness(&self) -> f32;
  fn chromosome(&self) -> &Chromosome;
  fn create(chromosome: Chromosome) -> Self;
}

pub trait SelectionMethod {
  fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
  where
    I: Individual;
}

pub trait CrossoverMethod {
  fn crossover(
    &self,
    rng: &mut dyn RngCore,
    parent_a: &Chromosome,
    parent_b: &Chromosome,
  ) -> Chromosome;
}

pub trait MutationMethod {
  fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct Statistics {
  min_fitness: f32,
  max_fitness: f32,
  avg_fitness: f32,
}

impl Statistics {
  fn new<I>(population: &[I]) -> Self
  where
    I: Individual,
  {
    assert!(!population.is_empty());

    let mut min_fitness = population[0].fitness();
    let mut max_fitness = min_fitness;
    let mut sum_fitness = 0.0;

    for individual in population {
      let fitness = individual.fitness();

      min_fitness = min_fitness.min(fitness);
      max_fitness = max_fitness.max(fitness);
      sum_fitness += fitness;
    }

    Self {
      min_fitness,
      max_fitness,
      avg_fitness: sum_fitness / (population.len() as f32),
    }
  }

  pub fn min_fitness(&self) -> f32 {
    self.min_fitness
  }

  pub fn max_fitness(&self) -> f32 {
    self.max_fitness
  }

  pub fn avg_fitness(&self) -> f32 {
    self.avg_fitness
  }
}
