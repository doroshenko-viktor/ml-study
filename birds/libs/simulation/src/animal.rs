use lib_genetic_algorithm::Chromosome;
use lib_neural_network::{LayerTopology, Network};
use nalgebra as na;
use nalgebra::{Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};

use crate::brain::Brain;
use crate::{Eye, Food};

#[derive(Debug)]
pub struct Animal {
  _position: Point2<f32>,
  _rotation: Rotation2<f32>,
  _speed: f32,
  _eye: Eye,
  _brain: Brain,
  _satiation: usize,
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

  pub fn get_angle(&self) -> f32 {
    self._rotation.angle()
  }

  pub fn get_rotation(&self) -> &Rotation2<f32> {
    &self._rotation
  }

  pub fn set_rotation(&mut self, value: Rotation2<f32>) {
    self._rotation = value
  }

  pub fn get_speed(&self) -> &f32 {
    &self._speed
  }

  pub fn set_speed(&mut self, value: f32) {
    self._speed = value
  }

  pub fn get_satiation(&self) -> usize {
    self._satiation
  }

  pub fn increase_satiation(&mut self, step: usize) {
    self._satiation += step
  }

  pub fn process_vision(&mut self, foods: &[Food]) -> Vec<f32> {
    self
      ._eye
      .process_vision(self._position, self._rotation, foods)
  }

  pub fn propagate(&mut self, inputs: Vec<f32>) -> Vec<f32> {
    self._brain.propagate(inputs)
  }

  pub fn random(rng: &mut dyn RngCore) -> Self {
    let eye = Eye::default();

    let brain = Brain::random(rng, &eye);

    Self {
      _position: rng.gen(),
      _rotation: rng.gen(),
      _speed: 0.002,
      _brain: brain,
      _eye: eye,
      _satiation: 0,
    }
  }

  pub fn as_chromosome(&self) -> Chromosome {
    self._brain.as_chromosome()
  }

  pub fn from_chromosome(
    chromosome: Chromosome,
    rng: &mut dyn RngCore,
  ) -> Self {
    let eye = Eye::default();
    let brain = Brain::from_chromosome(chromosome, &eye);

    Self::new(eye, brain, rng)
  }

  fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
    Self {
      _position: rng.gen(),
      _rotation: rng.gen(),
      _speed: 0.002,
      _eye: eye,
      _brain: brain,
      _satiation: 0,
    }
  }
}
