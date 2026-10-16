// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Parallel Balanced-Ternary Layer Topology & Vector Propagation )
// ============================================================================
//! # Ternary Layer (`layer.rs`)
//!
//! Organizes parallel `TernaryNeuron` units into discrete execution layers,
//! providing forward vector propagation across the network topology.

use crate::core::ternary::QuadState;
use super::neuron::TernaryNeuron;

/// A collection of parallel neurons forming a discrete layer.
#[derive(Debug, Clone)]
pub struct TernaryLayer {
    pub neurons: Vec<TernaryNeuron>,
}

impl TernaryLayer {
    /// Constructs a layer with `neuron_count` neurons taking inputs.
    pub fn new(
        inputs_per_neuron: usize,
        neuron_count: usize,
        threshold: i32,
    ) -> Self {
        let neurons = (0..neuron_count)
            .map(|_| TernaryNeuron::new(inputs_per_neuron, 0, threshold))
            .collect();
        Self { neurons }
    }

    /// Forward-propagates an input vector across all neurons in the layer.
    pub fn forward(&self, inputs: &[QuadState]) -> Vec<QuadState> {
        self.neurons.iter().map(|n| n.activate(inputs)).collect()
    }
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification )
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_forward_pass() {
        let layer = TernaryLayer::new(2, 3, 1);
        let out = layer.forward(&[QuadState::Proceed, QuadState::Proceed]);
        assert_eq!(out.len(), 3);
    }
}
