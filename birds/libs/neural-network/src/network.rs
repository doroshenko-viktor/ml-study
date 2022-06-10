use crate::{layer::Layer, LayerTopology};

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }

    fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let build_layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[0].neurons))
            .collect();

        Self {
            layers: build_layers,
        }
    }
}
