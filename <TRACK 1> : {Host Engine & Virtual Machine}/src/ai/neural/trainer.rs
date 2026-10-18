// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Episodic Neural Trainer Identity & Supervised Balanced-Ternary Training )
// ============================================================================
//! # Episodic Neural Trainer (`trainer.rs`)
//!
//! Dogfoods recorded execution scrolls and traditional logs as the initial
//! training dataset for the `NeuralBrain`. Maps trapped failpoints, restorator
//! patches, and final verification states into balanced-ternary weights.

use crate::ai::model::learner::EpisodicExperience;
use crate::core::ternary::QuadState;
use super::brain::NeuralBrain;
use super::curriculum::Curriculum;
use super::noise::NoiseEngine;

/// Detailed summary report of a neural training cycle.
#[derive(Debug, Clone, Default)]
pub struct TrainingReport {
    pub total_episodes: usize,
    pub archetypes_adapted: usize,
    pub synapses_adapted: usize,
    pub clean_proceed_count: usize,
    pub degraded_rollback_count: usize,
    pub noise_resilience_before: f64,
    pub noise_resilience_after: f64,
}

/// The supervised episodic neural network trainer.
#[derive(Debug, Clone, Default)]
pub struct NeuralTrainer {
    pub noise_engine: NoiseEngine,
}

impl NeuralTrainer {
    /// Creates a new [`NeuralTrainer`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Trains the [`NeuralBrain`] using a batch of episodic experiences.
    pub fn train(
        &self,
        brain: &mut NeuralBrain,
        experiences: &[EpisodicExperience],
    ) -> TrainingReport {
        let in_len = brain
            .layers
            .first()
            .and_then(|l| l.neurons.first())
            .map(|n| n.weights.len())
            .unwrap_or(64);
        let test_stimulus = vec![QuadState::Proceed; in_len];
        let res_before = brain.evaluate_noise_resilience(
            &test_stimulus,
            &self.noise_engine,
        );

        let mut report = TrainingReport {
            total_episodes: experiences.len(),
            noise_resilience_before: res_before,
            ..Default::default()
        };

        for (ep_idx, exp) in experiences.iter().enumerate() {
            if brain.layers.is_empty() {
                break;
            }

            match exp.final_state {
                QuadState::Proceed => report.clean_proceed_count += 1,
                QuadState::Rollback => report.degraded_rollback_count += 1,
                _ => {}
            }

            // Adapt input layer connections
            let l0_len = brain.layers[0].neurons.len();
            if l0_len > 0 {
                let neuron_idx = ep_idx % l0_len;
                let w_len = brain.layers[0].neurons[neuron_idx].weights.len();
                if w_len > 0 {
                    let weight_idx = (exp.repair_count + exp.failpoint_count)
                        % w_len;
                    brain.adapt_synapse(
                        0,
                        neuron_idx,
                        weight_idx,
                        exp.final_state,
                    );
                    report.synapses_adapted += 1;
                }
            }

            // Adapt output layer connection
            if brain.layers.len() > 1 {
                let last = brain.layers.len() - 1;
                let out_len = brain.layers[last].neurons.len();
                if out_len > 0 {
                    let out_neuron = 0;
                    let out_w_len = brain.layers[last].neurons[out_neuron]
                        .weights
                        .len();
                    if out_w_len > 0 {
                        let out_w = ep_idx % out_w_len;
                        brain.adapt_synapse(
                            last,
                            out_neuron,
                            out_w,
                            exp.final_state,
                        );
                        report.synapses_adapted += 1;
                    }
                }
            }
        }

        report.noise_resilience_after = brain.evaluate_noise_resilience(
            &test_stimulus,
            &self.noise_engine,
        );

        report
    }

    /// Trains the [`NeuralBrain`] across the 25-chapter NotebookLM curriculum.
    pub fn train_curriculum(
        &self,
        brain: &mut NeuralBrain,
        curriculum: &Curriculum,
    ) -> TrainingReport {
        let in_len = brain
            .layers
            .first()
            .and_then(|l| l.neurons.first())
            .map(|n| n.weights.len())
            .unwrap_or(64);
        let test_stimulus = vec![QuadState::Proceed; in_len];
        let res_before = brain.evaluate_noise_resilience(
            &test_stimulus,
            &self.noise_engine,
        );

        let mut report = TrainingReport {
            total_episodes: curriculum.len(),
            noise_resilience_before: res_before,
            ..Default::default()
        };

        for (ch_idx, chapter) in curriculum.chapters.iter().enumerate() {
            if brain.layers.is_empty() {
                break;
            }

            match chapter.target_state {
                QuadState::Proceed => report.clean_proceed_count += 1,
                QuadState::Rollback => report.degraded_rollback_count += 1,
                _ => {}
            }

            // Adapt input layer synapses
            let l0_len = brain.layers[0].neurons.len();
            if l0_len > 0 {
                let neuron_idx = ch_idx % l0_len;
                let w_len = brain.layers[0].neurons[neuron_idx].weights.len();
                if w_len > 0 {
                    let weight_idx = (ch_idx * 3 + chapter.file_count) % w_len;
                    brain.adapt_synapse(0, neuron_idx, weight_idx, chapter.target_state);
                    report.synapses_adapted += 1;
                }
            }

            // Adapt output / deliberation layer synapses
            let last = brain.layers.len() - 1;
            if last > 0 {
                let out_len = brain.layers[last].neurons.len();
                if out_len > 0 {
                    let out_neuron = ch_idx % out_len;
                    let out_w_len = brain.layers[last].neurons[out_neuron].weights.len();
                    if out_w_len > 0 {
                        let out_w = ch_idx % out_w_len;
                        brain.adapt_synapse(last, out_neuron, out_w, chapter.target_state);
                        report.synapses_adapted += 1;
                    }
                }
            }

            report.archetypes_adapted += 1;
        }

        report.noise_resilience_after = brain.evaluate_noise_resilience(
            &test_stimulus,
            &self.noise_engine,
        );

        report
    }

    /// Trains the network on canonical ideal OmniCode archetypes.
    /// Establishes the infant brain's foundational grammar and bijective trits.
    pub fn train_archetypes(
        &self,
        brain: &mut NeuralBrain,
        archetypes: &[&str],
    ) -> usize {
        let in_len = brain
            .layers
            .first()
            .and_then(|l| l.neurons.first())
            .map(|n| n.weights.len())
            .unwrap_or(64);
        let mut adapted = 0;
        for (a_idx, &text) in archetypes.iter().enumerate() {
            let stimulus = NeuralBrain::encode_stimulus(text, in_len);
            if brain.layers.is_empty() {
                continue;
            }
            let l0_len = brain.layers[0].neurons.len();
            for (t_idx, &st) in stimulus.iter().enumerate() {
                let neuron_idx = (a_idx + t_idx) % l0_len;
                let w_len = brain.layers[0].neurons[neuron_idx].weights.len();
                if w_len > 0 {
                    let w_idx = t_idx % w_len;
                    brain.adapt_synapse(0, neuron_idx, w_idx, st);
                    adapted += 1;
                }
            }
            // Reinforce output layer to Proceed (+1) for canonical archetypes
            if brain.layers.len() > 1 {
                let last = brain.layers.len() - 1;
                let out_len = brain.layers[last].neurons.len();
                if out_len > 0 {
                    let out_neuron = a_idx % out_len;
                    let out_w_len =
                        brain.layers[last].neurons[out_neuron].weights.len();
                    if out_w_len > 0 {
                        let out_w = a_idx % out_w_len;
                        brain.adapt_synapse(
                            last,
                            out_neuron,
                            out_w,
                            QuadState::Proceed,
                        );
                        adapted += 1;
                    }
                }
            }
        }
        adapted
    }
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification )
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;
    use std::path::PathBuf;

    #[test]
    fn test_neural_trainer_ingests_episodes() {
        let logger = DiagnosticLogger::new(false);
        let mut brain = NeuralBrain::new(&[3, 4, 2], 1, logger);
        let trainer = NeuralTrainer::new();

        let experiences = vec![
            EpisodicExperience {
                session_id: "test_sess_1".into(),
                prompt: "echo A".into(),
                final_state: QuadState::Proceed,
                repair_count: 0,
                failpoint_count: 0,
                scroll_path: PathBuf::from("test.scroll"),
                log_path: None,
            },
            EpisodicExperience {
                session_id: "test_sess_2".into(),
                prompt: "bad input".into(),
                final_state: QuadState::Rollback,
                repair_count: 1,
                failpoint_count: 1,
                scroll_path: PathBuf::from("test2.scroll"),
                log_path: None,
            },
        ];

        let report = trainer.train(&mut brain, &experiences);
        assert_eq!(report.total_episodes, 2);
        assert!(report.synapses_adapted > 0);
    }

    #[test]
    fn test_neural_trainer_archetypes() {
        let logger = DiagnosticLogger::new(false);
        let mut brain = NeuralBrain::new(&[8, 16, 4], 1, logger);
        let trainer = NeuralTrainer::new();
        let archetypes = ["Character 'A'", "Number 65"];
        let adapted = trainer.train_archetypes(&mut brain, &archetypes);
        assert!(adapted > 0);
    }
}
