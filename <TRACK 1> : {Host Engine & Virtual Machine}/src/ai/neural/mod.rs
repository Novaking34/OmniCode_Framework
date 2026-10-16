// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Neural Network Subsystem Identity, Architecture Manifest & Re-exports )
// ============================================================================
//! OmniCode Neural Network Subsystem
//!
//! Exposes balanced-ternary neural network engines, synaptic weights,
//! non-random structured noise engines, and episodic learning adapters.

pub mod brain;
pub mod layer;
pub mod network;
pub mod neuron;
pub mod noise;
pub mod synapse;
pub mod trainer;

// ============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Neural Network Components )
// ============================================================================
pub use brain::NeuralBrain;
pub use layer::TernaryLayer;
pub use neuron::TernaryNeuron;
pub use noise::{
    DegradationMode, DegradationStep, NoiseEngine, StructuredNoiseConfig,
};
pub use synapse::SynapticWeight;
pub use trainer::{NeuralTrainer, TrainingReport};

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Neural Network Subsystem Core Coordination )
// ============================================================================
// Neural network pathways actively linked to AICortex and diagnostics.

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// ============================================================================
// Neural network subsystem modularized, verified, and sealed.
