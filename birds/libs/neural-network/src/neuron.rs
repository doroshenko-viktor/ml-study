use rand::Rng;

#[derive(Debug)]
pub struct Neuron {
  bias: f32,
  weights: Vec<f32>,
}

impl Neuron {
  pub fn propagate(&self, inputs: &[f32]) -> f32 {
    assert_eq!(inputs.len(), self.weights.len());

    let mut output = inputs
      .iter()
      .zip(&self.weights)
      .map(|(input, weight)| input * weight)
      .sum::<f32>();
    output += self.bias;

    output.max(0f32)
  }

  pub fn random(output_size: usize) -> Self {
    let mut rng = rand::thread_rng();
    let bias = rng.gen_range(-1f32..=1f32);

    let weights = (0..output_size)
      .map(|_| rng.gen_range(-1f32..=1f32))
      .collect();

    Self { bias, weights }
  }

  pub fn get_bias(&self) -> f32 {
    self.bias
  }

  pub fn get_weights(&self) -> &Vec<f32> {
    &self.weights
  }

  pub fn from_weights(
    output_neurons: usize,
    weights: &mut dyn Iterator<Item = f32>,
  ) -> Self {
    let bias = weights
      .next()
      .expect("got not enough weights");

    let weights = (0..output_neurons)
      .map(|_| {
        weights
          .next()
          .expect("got not enough weights")
      })
      .collect();

    Self { bias, weights }
  }
}
