use crate::{Individual, SelectionMethod};
use rand::{prelude::SliceRandom, RngCore};
use wasm_bindgen::JsValue;
use web_sys::console;

pub struct RouletteWheelSelection;

impl Default for RouletteWheelSelection {
  fn default() -> Self {
    Self::new()
  }
}

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
    // console::log_1(&JsValue::from(format!(
    //   "population length {}",
    //   population.len()
    // )));

    let x = population
      .choose_weighted(rng, |ind| ind.fitness())
      .expect("Empty population");

    // console::log_1(&JsValue::from(format!(
    //   "chosen {}",
    //   x.fitness()
    // )));

    x
  }
}
