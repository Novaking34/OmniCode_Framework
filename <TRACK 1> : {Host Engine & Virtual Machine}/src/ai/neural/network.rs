// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Neural Network Facade & Backward Compatibility Re-exports )
// ============================================================================
//! OmniCode Balanced-Ternary Neural Network (`network.rs`)
//!
//! Re-exports the modular neural network components:
//! - [`synapse`]: Discrete ternary synaptic weights $\{-1, 0, +1\}$.
//! - [`neuron`]: `TernaryNeuron` with threshold activation $\Psi(\Sigma)$.
//! - [`layer`]: `TernaryLayer` forward propagation.
//! - [`brain`]: `NeuralBrain` multi-layer cognitive network.
//! - [`noise`]: Non-random structured noise and degradation engine.
//! - [`trainer`]: Supervised episodic neural trainer.

pub use super::synapse::SynapticWeight;
pub use super::neuron::TernaryNeuron;
pub use super::layer::TernaryLayer;
pub use super::brain::NeuralBrain;
pub use super::noise::{
    DegradationMode, DegradationStep, NoiseEngine, StructuredNoiseConfig,
};
pub use super::trainer::{NeuralTrainer, TrainingReport};
