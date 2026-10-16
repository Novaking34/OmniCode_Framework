// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Balanced-Ternary Neuron Unit Identity & Threshold Dynamics )
// ============================================================================
//! # Balanced-Ternary Neuron (`neuron.rs`)
//!
//! Models an artificial neuron computing excitation $\Sigma = \sum w_i x_i + b$
//! and evaluating activation $\Psi(\Sigma)$ against threshold $\theta$.

use crate::core::ternary::QuadState;
use super::synapse::SynapticWeight;

/// Individual balanced-ternary artificial neuron.
#[derive(Debug, Clone)]
pub struct TernaryNeuron {
    /// Incoming synaptic connection weights.
    pub weights: Vec<SynapticWeight>,
    /// Intrinsic neuron threshold bias (-1, 0, +1).
    pub bias: i32,
    /// Sensitivity threshold $\theta$ governing activation boundary.
    pub threshold: i32,
}

impl TernaryNeuron {
    /// Creates a new ternary neuron with initialized weights.
    pub fn new(inputs: usize, bias: i32, threshold: i32) -> Self {
        Self {
            weights: vec![SynapticWeight::Neutral; inputs],
            bias,
            threshold: threshold.max(1),
        }
    }

    /// Computes the net balanced ternary excitation and fires $\Psi(\Sigma)$.
    pub fn activate(&self, inputs: &[QuadState]) -> QuadState {
        let mut sum = self.bias;
        for (w, &inp) in self.weights.iter().zip(inputs.iter()) {
            let x = match inp {
                QuadState::Proceed => 1,
                QuadState::Await => 0,
                QuadState::Rollback => -1,
                QuadState::Halt => return QuadState::Halt,
            };
            sum += w.to_trit() * x;
        }

        if sum > self.threshold {
            QuadState::Proceed
        } else if sum < -self.threshold {
            QuadState::Rollback
        } else {
            QuadState::Await
        }
    }

    /// Adapts a specific synaptic weight to target state.
    pub fn adapt(&mut self, weight_idx: usize, target: SynapticWeight) {
        if let Some(w) = self.weights.get_mut(weight_idx) {
            *w = target;
        }
    }

    /// Adapts sensitivity threshold θ via homeostatic metaplasticity (BCM).
    /// Prevents runaway excitation saturation or sensory silence.
    pub fn adapt_threshold(&mut self, mean_excitation: f64) {
        if mean_excitation.abs() > (self.threshold as f64 * 1.5) {
            self.threshold = (self.threshold + 1).min(10);
        } else if mean_excitation.abs() < 0.2 && self.threshold > 1 {
            self.threshold -= 1;
        }
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
    fn test_neuron_activation() {
        let mut neuron = TernaryNeuron::new(2, 0, 1);
        neuron.weights[0] = SynapticWeight::Excitatory;
        neuron.weights[1] = SynapticWeight::Excitatory;

        assert_eq!(
            neuron.activate(&[QuadState::Proceed, QuadState::Proceed]),
            QuadState::Proceed
        );
        assert_eq!(
            neuron.activate(&[QuadState::Rollback, QuadState::Proceed]),
            QuadState::Await
        );
    }

    #[test]
    fn test_neuron_metaplasticity_threshold() {
        let mut neuron = TernaryNeuron::new(2, 0, 2);
        // High excitation increases threshold
        neuron.adapt_threshold(4.0);
        assert_eq!(neuron.threshold, 3);
        // Low excitation lowers threshold
        neuron.adapt_threshold(0.05);
        assert_eq!(neuron.threshold, 2);
    }
}
