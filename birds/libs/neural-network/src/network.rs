use crate::{layer::Layer, LayerTopology};

#[derive(Debug)]
pub struct Network {
    _layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self._layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }

    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let build_layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[0].neurons))
            .collect();

        Self {
            _layers: build_layers,
        }
    }
}
