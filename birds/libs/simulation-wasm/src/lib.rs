use lib_simulation as sim;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Simulation {
    _rng: ThreadRng,
    _sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self {
            _rng: rng,
            _sim: sim,
        }
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
            rotation: animal.rotation().angle(),
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
        let animals = world.get_animals().iter().map(Animal::from).collect();
        let foods = world.get_foods().iter().map(Food::from).collect();

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

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }
