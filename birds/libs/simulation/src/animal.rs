use nalgebra as na;
use nalgebra::{Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};

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

    pub fn step(&mut self) {
        self._position += self._rotation * Vector2::new(self._speed, 0.0);
        self._position.x = na::wrap(self._position.x, 0.0, 1.0);
        self._position.y = na::wrap(self._position.y, 0.0, 1.0);
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
