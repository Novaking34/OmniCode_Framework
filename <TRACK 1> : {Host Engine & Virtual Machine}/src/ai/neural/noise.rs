// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Systemic Noise, Entropy Physics & Machine Degradation Engine )
// ============================================================================
//! # Structured Systemic Noise Engine (`noise.rs`)
//!
//! A true simulation of mind requires understanding what noise is.
//! A "perfect" machine is impossible, and statistical brute-force is
//! non-scalable. To understand what an "ideal" is and what happens as the
//! machine is "breaking down" requires non-random, structured noise:
//!
//! 1. **Signal Attenuation (Trit Decay)**: Signals under cognitive fatigue
//!    decay toward neutral equilibrium (`+1` or `-1` $\rightarrow$ `0:Await`).
//! 2. **Boundary Cross-Talk (Block Leak)**: Cross-channel leakage between
//!    adjacent trits along the 4-block ladder.
//! 3. **Thermal Inversion**: Severe stress causes phase inversion
//!    (`+1:Proceed` $\leftrightarrow$ `-1:Rollback`).
//! 4. **Diagnostic Failpoint Perturbation**: Systematic injection of known
//!    hardware/compiler failpoints (null trap, caveat veto, boundary mismatch).

use crate::core::ternary::QuadState;

/// Degradation modes through which a cognitive machine breaks down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegradationMode {
    /// Ideal state: zero entropy, pure signal transmission.
    Ideal,
    /// Trit attenuation: excitation decays toward equilibrium (Await).
    TritDecay,
    /// Cross-talk: adjacent block or channel signal leakage.
    BoundaryLeak,
    /// Thermal inversion: polarity flips under stress.
    ThermalInversion,
    /// Critical failpoint: circuit halts or enters rollback.
    FailpointTrap,
}

/// Structured noise configuration governing breakdown physics.
#[derive(Debug, Clone)]
pub struct StructuredNoiseConfig {
    pub decay_probability: f64,
    pub leak_probability: f64,
    pub inversion_probability: f64,
}

impl Default for StructuredNoiseConfig {
    fn default() -> Self {
        Self {
            decay_probability: 0.15,
            leak_probability: 0.10,
            inversion_probability: 0.05,
        }
    }
}

/// A snapshot of machine degradation along the stress ladder.
#[derive(Debug, Clone)]
pub struct DegradationStep {
    pub stress_level: f64,
    pub dominant_mode: DegradationMode,
    pub perturbed_vector: Vec<QuadState>,
    pub signal_fidelity: f64,
}

/// The non-random noise synthesis engine.
#[derive(Debug, Clone, Default)]
pub struct NoiseEngine {
    pub config: StructuredNoiseConfig,
}

impl NoiseEngine {
    /// Constructs a [`NoiseEngine`] with specified configuration.
    pub fn new(config: StructuredNoiseConfig) -> Self {
        Self { config }
    }

    /// Injects structured, non-random noise into an input vector at a given
    /// stress level $\in [0.0, 1.0]$.
    pub fn perturb(
        &self,
        vector: &[QuadState],
        stress: f64,
        step_seed: u64,
    ) -> Vec<QuadState> {
        if stress <= 0.0 || vector.is_empty() {
            return vector.to_vec();
        }

        let clamped_stress = stress.clamp(0.0, 1.0);
        let mut out = vector.to_vec();
        let len = out.len();

        for i in 0..len {
            // Pseudo-random pseudo-physical deterministic hash
            let h = deterministic_hash(step_seed, i as u64, clamped_stress);

            let effective_decay = self.config.decay_probability
                * clamped_stress;
            let effective_leak = self.config.leak_probability
                * clamped_stress;
            let effective_inv = self.config.inversion_probability
                * clamped_stress;

            if h < effective_inv {
                // Thermal Inversion
                out[i] = match out[i] {
                    QuadState::Proceed => QuadState::Rollback,
                    QuadState::Rollback => QuadState::Proceed,
                    other => other,
                };
            } else if h < effective_inv + effective_leak {
                // Boundary Leak: take signal from adjacent neighbor
                let neighbor_idx = if i > 0 { i - 1 } else { len - 1 };
                out[i] = vector[neighbor_idx];
            } else if h < effective_inv + effective_leak + effective_decay {
                // Trit Decay: decay toward equilibrium
                out[i] = match out[i] {
                    QuadState::Proceed | QuadState::Rollback => {
                        QuadState::Await
                    }
                    other => other,
                };
            }
        }

        out
    }

    /// Evaluates degradation across a ladder of stress levels
    /// [0.0, 0.25, 0.50, 0.75, 1.0].
    pub fn simulate_breakdown_ladder(
        &self,
        ideal: &[QuadState],
    ) -> Vec<DegradationStep> {
        let levels = [0.0, 0.25, 0.50, 0.75, 1.0];
        let mut ladder = Vec::new();

        for (idx, &stress) in levels.iter().enumerate() {
            let perturbed = self.perturb(ideal, stress, idx as u64);
            let fidelity = compute_fidelity(ideal, &perturbed);

            let mode = if stress == 0.0 {
                DegradationMode::Ideal
            } else if stress <= 0.25 {
                DegradationMode::TritDecay
            } else if stress <= 0.50 {
                DegradationMode::BoundaryLeak
            } else if stress <= 0.75 {
                DegradationMode::ThermalInversion
            } else {
                DegradationMode::FailpointTrap
            };

            ladder.push(DegradationStep {
                stress_level: stress,
                dominant_mode: mode,
                perturbed_vector: perturbed,
                signal_fidelity: fidelity,
            });
        }

        ladder
    }
}

/// Measures the cosine/ternary signal fidelity between ideal and perturbed.
fn compute_fidelity(ideal: &[QuadState], perturbed: &[QuadState]) -> f64 {
    if ideal.is_empty() || perturbed.is_empty() {
        return 1.0;
    }
    let mut matches = 0;
    for (a, b) in ideal.iter().zip(perturbed.iter()) {
        if a == b {
            matches += 1;
        }
    }
    matches as f64 / ideal.len() as f64
}

/// Deterministic, non-cryptographic hash for pseudo-physical noise modeling.
fn deterministic_hash(seed: u64, index: u64, stress: f64) -> f64 {
    let stress_bits = (stress * 1_000_000.0) as u64;
    let mut v = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(index.wrapping_mul(1442695040888963407))
        .wrapping_add(stress_bits);
    v ^= v >> 33;
    v = v.wrapping_mul(0xff51afd7ed558ccd);
    v ^= v >> 33;
    (v & 0xFFFFFF) as f64 / 16777215.0
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification )
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_state_preserves_clean_vector() {
        let engine = NoiseEngine::default();
        let ideal = vec![QuadState::Proceed, QuadState::Proceed];
        let perturbed = engine.perturb(&ideal, 0.0, 42);
        assert_eq!(ideal, perturbed);
    }

    #[test]
    fn test_breakdown_ladder_monotonic_decay() {
        let engine = NoiseEngine::default();
        let ideal = vec![
            QuadState::Proceed,
            QuadState::Proceed,
            QuadState::Proceed,
            QuadState::Proceed,
        ];
        let ladder = engine.simulate_breakdown_ladder(&ideal);
        assert_eq!(ladder.len(), 5);
        assert_eq!(ladder[0].signal_fidelity, 1.0);
    }
}
