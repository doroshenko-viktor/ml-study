use std::panic;

use lib_simulation as sim;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use serde::Serialize;
use wasm_bindgen::prelude::*;
// use web_sys::console;

#[wasm_bindgen]
pub struct Simulation {
  _rng: ThreadRng,
  _sim: sim::Simulation,
}

impl Default for Simulation {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
impl Simulation {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut rng = thread_rng();
    let sim = sim::Simulation::random(&mut rng);

    Self {
      _rng: rng,
      _sim: sim,
    }
  }

  pub fn train(&mut self) -> String {
    let stats = self._sim.train(&mut self._rng);

    format!(
      "min={:.2}, max={:.2}, avg={:.2}",
      stats.min_fitness(),
      stats.max_fitness(),
      stats.avg_fitness()
    )
  }

  pub fn world(&self) -> JsValue {
    let world = World::from(self._sim.world());
    let x = JsValue::from_serde(&world).unwrap();
    // console::log_1(&x);
    x
  }

  pub fn step(&mut self) {
    self._sim.step(&mut self._rng);
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
  pub x: f32,
  pub y: f32,
  rotation: f32,
}

impl From<&sim::Animal> for Animal {
  fn from(animal: &sim::Animal) -> Self {
    Self {
      x: animal.position().x,
      y: animal.position().y,
      rotation: animal.get_rotation().angle(),
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
  pub animals: Vec<Animal>,
  pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
  fn from(world: &sim::World) -> Self {
    let animals = world
      .get_animals()
      .iter()
      .map(Animal::from)
      .collect();
    let foods = world
      .get_foods()
      .iter()
      .map(Food::from)
      .collect();

    Self { animals, foods }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct Food {
  pub x: f32,
  pub y: f32,
}

impl From<&sim::Food> for Food {
  fn from(food: &sim::Food) -> Self {
    Self {
      x: food.get_position().x,
      y: food.get_position().y,
    }
  }
}
