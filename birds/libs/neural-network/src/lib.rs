use rand::Rng;

pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(inputs: usize, outputs: usize) -> Self {
        let neurons = (0..outputs).map(|_| Neuron::random(inputs)).collect();

        Self { neurons }
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn random(output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1f32..=1f32);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1f32..=1f32))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let mut output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        output += self.bias;

        output.max(0f32)
    }
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }

    fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        // let mut build_layers = Vec::new();

        // for [first, second] in layers.windows(2) {
        //     let input_neurons = first.neurons;
        //     let output_neurons = second.neurons;
        //     build_layers.push(Layer::random(input_neurons, output_neurons));
        // }

        let build_layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[0].neurons))
            .collect();

        Self {
            layers: build_layers,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
