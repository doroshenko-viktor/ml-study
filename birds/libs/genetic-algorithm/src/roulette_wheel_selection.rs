use crate::{Individual, SelectionMethod};
use rand::{prelude::SliceRandom, RngCore};

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        return population
            .choose_weighted(rng, |ind| ind.fitness())
            .expect("Empty population");
    }
}
