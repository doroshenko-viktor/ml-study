use rand::RngCore;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::{
  CrossoverMethod, Individual, MutationMethod, SelectionMethod, Statistics,
};

pub struct GeneticAlgorithm<S>
where
  S: SelectionMethod,
{
  selection_method: S,
  crossover_method: Box<dyn CrossoverMethod>,
  mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
  S: SelectionMethod,
{
  pub fn new(
    selection_method: S,
    crossover_method: impl CrossoverMethod + 'static,
    mutation_method: impl MutationMethod + 'static,
  ) -> Self {
    Self {
      selection_method,
      crossover_method: Box::new(crossover_method),
      mutation_method: Box::new(mutation_method),
    }
  }

  pub fn evolve<I>(
    &self,
    rng: &mut dyn RngCore,
    population: &[I],
  ) -> (Vec<I>, Statistics)
  where
    I: Individual,
  {
    assert!(!population.is_empty());

    let new_population: Vec<I> = (0..population.len())
      .map(|_| {
        let parent_a = self
          .selection_method
          .select(rng, population)
          .chromosome();

        let parent_b = self
          .selection_method
          .select(rng, population)
          .chromosome();

        let mut child = self
          .crossover_method
          .crossover(rng, parent_a, parent_b);

        self
          .mutation_method
          .mutate(rng, &mut child);

        I::create(child)
      })
      .collect();

    let stats = Statistics::new(population);

    (new_population, stats)
  }
}
