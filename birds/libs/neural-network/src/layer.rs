use crate::neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(inputs: usize, outputs: usize) -> Self {
        let neurons = (0..outputs).map(|_| Neuron::random(inputs)).collect();

        Self { neurons }
    }
}
