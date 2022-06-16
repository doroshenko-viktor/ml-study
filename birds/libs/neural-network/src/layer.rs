use crate::neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
  neurons: Vec<Neuron>,
}

impl Layer {
  pub fn get_neurons(&self) -> &[Neuron] {
    &self.neurons
  }

  pub fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
    self
      .neurons
      .iter()
      .map(|neuron| neuron.propagate(&inputs))
      .collect()
  }

  pub fn random(inputs: usize, outputs: usize) -> Self {
    let neurons = (0..outputs)
      .map(|_| Neuron::random(inputs))
      .collect();

    Self { neurons }
  }

  pub fn from_weights(
    input_size: usize,
    output_size: usize,
    weights: &mut dyn Iterator<Item = f32>,
  ) -> Self {
    let neurons = (0..output_size)
      .map(|_| Neuron::from_weights(input_size, weights))
      .collect();

    Self { neurons }
  }
}
