use crate::{layer::Layer, neuron::Neuron, LayerTopology};
use std::iter::once;

#[derive(Debug)]
pub struct Network {
  _layers: Vec<Layer>,
}

impl Network {
  pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
    self
      ._layers
      .iter()
      .fold(inputs, |inputs, layer| {
        layer.propagate(&inputs)
      })
  }

  pub fn random(layers: &[LayerTopology]) -> Self {
    assert!(layers.len() > 1);

    let build_layers = layers
      .windows(2)
      .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
      .collect();

    Self {
      _layers: build_layers,
    }
  }

  pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
    self
      ._layers
      .iter()
      .flat_map(|layer| layer.get_neurons().iter())
      .flat_map(|neuron| {
        once(neuron.get_bias()).chain(neuron.get_weights().iter().cloned())
      })
  }

  pub fn from_weights(
    layers: &[LayerTopology],
    weights: impl IntoIterator<Item = f32>,
  ) -> Self {
    assert!(layers.len() > 1);

    let mut weights = weights.into_iter();

    let layers = layers
      .windows(2)
      .map(|layers| {
        Layer::from_weights(
          layers[0].neurons,
          layers[1].neurons,
          &mut weights,
        )
      })
      .collect();

    if weights.next().is_some() {
      panic!("got too many weights");
    }

    Self { _layers: layers }
  }
}
