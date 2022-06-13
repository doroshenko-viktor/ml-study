use nalgebra::Point2;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Food {
    _position: Point2<f32>,
}

impl Food {
    pub fn new(position: Point2<f32>) -> Self {
        Food {
            _position: position,
        }
    }

    pub fn get_position(&self) -> &Point2<f32> {
        &self._position
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self._position = position
    }

    pub fn randomize(&mut self, rng: &mut dyn RngCore) {
        self._position.x = rng.gen();
        self._position.y = rng.gen();
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            _position: rng.gen(),
        }
    }
}
