use nalgebra::{Point2, Rotation2};
use rand::{RngCore, Rng};

#[derive(Debug)]
pub struct Animal {
    _position: Point2<f32>,
    _rotation: Rotation2<f32>,
    _speed: f32,
}

impl Animal {
    pub fn position(&self) -> &Point2<f32> {
        &self._position
    }

    pub fn rotation(&self) -> &Rotation2<f32> {
        &self._rotation
    }

    pub fn speed(&self) -> &f32 {
        &self._speed
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            _position: rng.gen(),
            _rotation: rng.gen(),
            _speed: 0.002,
        }
    }
}
