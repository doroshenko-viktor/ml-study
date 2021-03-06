use rand::{Rng, RngCore};

use crate::{chromosome::Chromosome, CrossoverMethod};

pub struct UniformCrossover;

impl Default for UniformCrossover {
  fn default() -> Self {
    Self::new()
  }
}

impl UniformCrossover {
  pub fn new() -> Self {
    Self
  }
}

impl CrossoverMethod for UniformCrossover {
  fn crossover(
    &self,
    rng: &mut dyn RngCore,
    parent_a: &Chromosome,
    parent_b: &Chromosome,
  ) -> Chromosome {
    assert_eq!(parent_a.len(), parent_b.len());

    let parent_a = parent_a.iter();
    let parent_b = parent_b.iter();

    parent_a
      .zip(parent_b)
      .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
      .collect()
  }
}
