use nalgebra::Point2;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Food {
    _position: Point2<f32>,
}

impl Food {
    pub fn position(&self) -> &Point2<f32> {
        &self._position
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            _position: rng.gen(),
        }
    }
}
