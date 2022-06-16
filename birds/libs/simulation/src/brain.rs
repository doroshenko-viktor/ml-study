use lib_genetic_algorithm::Chromosome;
use lib_neural_network::{LayerTopology, Network};
use rand::RngCore;

use crate::*;

#[derive(Debug)]
pub struct Brain {
  nn: Network,
}

impl Brain {
  pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
    Self {
      nn: Network::random(&Self::topology(eye)),
    }
  }

  pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
    self.nn.propagate(inputs)
  }

  pub fn as_chromosome(&self) -> Chromosome {
    self.nn.weights().collect()
  }

  fn topology(eye: &Eye) -> [LayerTopology; 3] {
    [
      LayerTopology {
        neurons: eye.cells(),
      },
      LayerTopology {
        neurons: 2 * eye.cells(),
      },
      LayerTopology { neurons: 2 },
    ]
  }

  pub fn from_chromosome(chromosome: Chromosome, eye: &Eye) -> Self {
    Self {
      nn: Network::from_weights(&Self::topology(eye), chromosome),
    }
  }
}
