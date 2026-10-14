// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Balanced-Ternary Neurodynamics Identity, Synaptic Invariants & Axioms )
// ==============================================================================
//! # OmniCode Balanced-Ternary Neural Network (`network.rs`)
//!
//! Implements a homoiconic, balanced-ternary artificial neural network operating
//! directly over the fourfold `QuadState` {-1, 0, +1, NULL}.
//!
//! ## Mathematical Principles:
//! - **Synaptic Weights**: Discrete trits $w \in \{-1, 0, +1\}$ representing inhibitory,
//!   neutral, and excitatory connections.
//! - **Accumulation**: $\Sigma = \sum w_i x_i + b$ over balanced integer sums.
//! - **Threshold Activation $\Psi(\Sigma)$**:
//!   - $\Sigma > \theta \rightarrow +1:\text{PROCEED}$ (Affirmative excitation)
//!   - $-\theta \le \Sigma \le \theta \rightarrow 0:\text{AWAIT}$ (Equilibrium / neutral hold)
//!   - $\Sigma < -\theta \rightarrow -1:\text{ROLLBACK}$ (Inhibitory veto / caveat trip)
//!   - Circuit rupture / entropy $\rightarrow \text{NULL}:\text{HALT}$ (Absolute cessation)

use crate::core::ternary::QuadState;
use crate::diagnostics::logger::DiagnosticLogger;

/// Discrete ternary synaptic connection weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynapticWeight {
    /// Inhibitory connection (-1)
    Inhibitory,
    /// Neutral / zero-bias connection (0)
    Neutral,
    /// Excitatory connection (+1)
    Excitatory,
}

impl SynapticWeight {
    /// Returns the integer trit scalar representation (-1, 0, +1).
    pub fn to_trit(self) -> i32 {
        match self {
            Self::Inhibitory => -1,
            Self::Neutral => 0,
            Self::Excitatory => 1,
        }
    }

    /// Constructs a synaptic weight from a QuadState.
    pub fn from_quad_state(state: QuadState) -> Self {
        match state {
            QuadState::Proceed => Self::Excitatory,
            QuadState::Await | QuadState::Halt => Self::Neutral,
            QuadState::Rollback => Self::Inhibitory,
        }
    }
}


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Neural Unit Definitions, Layer Topology & Brain State Initialization )
// ==============================================================================
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
}

/// A collection of parallel neurons forming a discrete layer.
#[derive(Debug, Clone)]
pub struct TernaryLayer {
    pub neurons: Vec<TernaryNeuron>,
}

impl TernaryLayer {
    /// Constructs a layer with `neuron_count` neurons each taking `inputs_per_neuron`.
    pub fn new(inputs_per_neuron: usize, neuron_count: usize, threshold: i32) -> Self {
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

/// Multi-layer neural brain coordinating sensory input to cortical action.
#[derive(Debug, Clone)]
pub struct NeuralBrain {
    /// Sequential network layers (Input -> Hidden -> Output).
    pub layers: Vec<TernaryLayer>,
    /// Diagnostic telemetry logger.
    pub logger: DiagnosticLogger,
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Forward Propagation, Experience Ingestion & Dogfooding Weight Updates )
// ==============================================================================
impl NeuralBrain {
    /// Constructs a new balanced-ternary [`NeuralBrain`].
    pub fn new(topology: &[usize], threshold: i32, logger: DiagnosticLogger) -> Self {
        assert!(topology.len() >= 2, "Topology requires at least input and output layers");
        let mut layers = Vec::new();
        for i in 0..(topology.len() - 1) {
            layers.push(TernaryLayer::new(topology[i], topology[i + 1], threshold));
        }
        Self { layers, logger }
    }

    /// Evaluates sensory inputs through all layers, yielding output action QuadStates.
    pub fn think(&self, inputs: &[QuadState]) -> Vec<QuadState> {
        let mut current = inputs.to_vec();
        for (idx, layer) in self.layers.iter().enumerate() {
            current = layer.forward(&current);
            self.logger.log(
                "NEURAL_BRAIN",
                format!("Layer {} forward pass: {:?}", idx + 1, current),
            );
        }
        current
    }

    /// Dogfoods past execution experience to adapt synaptic weights.
    ///
    /// Reinforces positive associations (`+1:PROCEED`) and prunes paths that led
    /// to failpoints or rollbacks (`-1:ROLLBACK`).
    pub fn adapt_synapse(
        &mut self,
        layer_idx: usize,
        neuron_idx: usize,
        weight_idx: usize,
        target_state: QuadState,
    ) {
        if let Some(layer) = self.layers.get_mut(layer_idx) {
            if let Some(neuron) = layer.neurons.get_mut(neuron_idx) {
                if let Some(w) = neuron.weights.get_mut(weight_idx) {
                    *w = SynapticWeight::from_quad_state(target_state);
                    self.logger.log(
                        "NEURAL_BRAIN",
                        format!(
                            "Synapse adapted at L{}:N{}:W{} -> {:?}",
                            layer_idx, neuron_idx, weight_idx, w
                        ),
                    );
                }
            }
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Neurodynamic Verification Proofs & Brain Integration Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_neuron_activation_dynamics() {
        let mut neuron = TernaryNeuron::new(3, 0, 1);
        neuron.weights[0] = SynapticWeight::Excitatory; // +1
        neuron.weights[1] = SynapticWeight::Excitatory; // +1
        neuron.weights[2] = SynapticWeight::Inhibitory; // -1

        // 1. Two proceed inputs overcome threshold: +1 + 1 + 0 = 2 > 1 => Proceed
        let res1 = neuron.activate(&[QuadState::Proceed, QuadState::Proceed, QuadState::Await]);
        assert_eq!(res1, QuadState::Proceed);

        // 2. Equilibrium: +1 + 0 - 1 = 0 => Await
        let res2 = neuron.activate(&[QuadState::Proceed, QuadState::Await, QuadState::Proceed]);
        assert_eq!(res2, QuadState::Await);

        // 3. Inhibitory veto: 0 + 0 - 1 = -1 < 0 (with negative inputs): -1 - 1 = -2 < -1 => Rollback
        let res3 = neuron.activate(&[QuadState::Rollback, QuadState::Rollback, QuadState::Await]);
        assert_eq!(res3, QuadState::Rollback);
    }

    #[test]
    fn test_neural_brain_forward_pass_and_dogfooding_adaptation() {
        let logger = DiagnosticLogger::new(false);
        let mut brain = NeuralBrain::new(&[3, 4, 2], 1, logger);

        // Initial think pass on neutral equilibrium inputs
        let out1 = brain.think(&[QuadState::Proceed, QuadState::Proceed, QuadState::Proceed]);
        assert_eq!(out1.len(), 2);

        // Adapt synaptic connections to dogfood verified execution
        brain.adapt_synapse(0, 0, 0, QuadState::Proceed);
        brain.adapt_synapse(0, 0, 1, QuadState::Proceed);
        brain.adapt_synapse(1, 0, 0, QuadState::Proceed);

        let out2 = brain.think(&[QuadState::Proceed, QuadState::Proceed, QuadState::Proceed]);
        assert_eq!(out2.len(), 2);
    }
}
