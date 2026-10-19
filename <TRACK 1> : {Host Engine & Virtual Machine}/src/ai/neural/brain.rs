// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Layer Balanced-Ternary Neural Brain Identity & Cortical Control )
// ============================================================================
//! # Balanced-Ternary Neural Brain (`brain.rs`)
//!
//! Multi-layer balanced-ternary cognitive network coordinating sensory input,
//! forward thinking pass, synaptic adaptation, and noise resilience testing.

use crate::core::ternary::QuadState;
use crate::diagnostics::logger::DiagnosticLogger;
use super::layer::TernaryLayer;
use super::noise::NoiseEngine;
use super::synapse::SynapticWeight;

/// Multi-layer neural brain coordinating sensory input to cortical action.
#[derive(Debug, Clone)]
pub struct NeuralBrain {
    /// Sequential network layers (Input -> Hidden -> Output).
    pub layers: Vec<TernaryLayer>,
    /// Diagnostic telemetry logger.
    pub logger: DiagnosticLogger,
}

impl NeuralBrain {
    /// Constructs a new balanced-ternary [`NeuralBrain`].
    pub fn new(
        topology: &[usize],
        threshold: i32,
        logger: DiagnosticLogger,
    ) -> Self {
        assert!(
            topology.len() >= 2,
            "Topology requires at least input and output layers"
        );
        let mut layers = Vec::new();
        for i in 0..(topology.len() - 1) {
            layers.push(TernaryLayer::new(
                topology[i],
                topology[i + 1],
                threshold,
            ));
        }
        Self { layers, logger }
    }

    /// Evaluates sensory inputs through all layers, yielding action QuadStates.
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

    /// Evaluates sensory inputs under structured non-random noise stress,
    /// returning both the perturbed sensory vector and the final output.
    pub fn think_with_noise(
        &self,
        inputs: &[QuadState],
        noise: &NoiseEngine,
        stress: f64,
    ) -> (Vec<QuadState>, Vec<QuadState>) {
        let perturbed = noise.perturb(inputs, stress, 1337);
        let output = self.think(&perturbed);
        (perturbed, output)
    }

    /// Dogfoods past execution experience to adapt synaptic weights.
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

    /// Measures the network resilience against degradation noise $\in [0, 1]$.
    pub fn evaluate_noise_resilience(
        &self,
        clean_inputs: &[QuadState],
        noise: &NoiseEngine,
    ) -> f64 {
        let clean_output = self.think(clean_inputs);
        let ladder = noise.simulate_breakdown_ladder(clean_inputs);
        if ladder.is_empty() {
            return 1.0;
        }

        let mut agreement_sum = 0.0;
        for step in &ladder {
            let noisy_out = self.think(&step.perturbed_vector);
            let mut matches = 0;
            for (a, b) in clean_output.iter().zip(noisy_out.iter()) {
                if a == b {
                    matches += 1;
                }
            }
            if !clean_output.is_empty() {
                agreement_sum += matches as f64 / clean_output.len() as f64;
            }
        }

        agreement_sum / ladder.len() as f64
    }

    /// Bijectively encodes an input string into a balanced-ternary stimulus
    /// vector of the requested target length using the Byte-Pair Ternary Trie
    /// with zero OOV drops and balanced-ternary majority pooling.
    pub fn encode_stimulus(text: &str, target_len: usize) -> Vec<QuadState> {
        let tree = super::tokenizer::TernaryByteTree::new();
        tree.encode_to_sensory_vector(text, target_len)
    }

    /// Returns the counts of (excitatory, neutral, inhibitory) synapses.
    pub fn synaptic_counts(&self) -> (usize, usize, usize) {
        let mut exc = 0;
        let mut neu = 0;
        let mut inh = 0;
        for layer in &self.layers {
            for neuron in &layer.neurons {
                for w in &neuron.weights {
                    match w {
                        SynapticWeight::Excitatory => exc += 1,
                        SynapticWeight::Neutral => neu += 1,
                        SynapticWeight::Inhibitory => inh += 1,
                    }
                }
            }
        }
        (exc, neu, inh)
    }

    /// Persists the network's synaptic weights and thresholds to disk.
    pub fn save_weights(&self, path: &std::path::Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut lines = Vec::new();
        lines.push("# OMNICODE NEURAL BRAIN WEIGHTS (CPI-SI BASE)".into());
        lines.push(format!("LAYERS:{}", self.layers.len()));
        for (l_idx, layer) in self.layers.iter().enumerate() {
            for (n_idx, neuron) in layer.neurons.iter().enumerate() {
                lines.push(format!(
                    "N:{}:{}:{}:{}",
                    l_idx, n_idx, neuron.threshold, neuron.bias
                ));
                for (w_idx, w) in neuron.weights.iter().enumerate() {
                    lines.push(format!(
                        "W:{}:{}:{}:{}",
                        l_idx, n_idx, w_idx, w.to_trit()
                    ));
                }
            }
        }
        std::fs::write(path, lines.join("\n")).map_err(|e| e.to_string())
    }

    /// Loads synaptic weights and thresholds from a persisted file.
    pub fn load_weights(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| e.to_string())?;
        if let Some(first_layers) = content.lines().map(|l| l.trim()).find(|l| l.starts_with("LAYERS:")) {
            if let Ok(layer_cnt) = first_layers.trim_start_matches("LAYERS:").parse::<usize>() {
                if layer_cnt != self.layers.len() {
                    return Err(format!(
                        "Incompatible weight topology: file has {} layers, brain has {}",
                        layer_cnt,
                        self.layers.len()
                    ));
                }
            }
        }
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = trimmed.split(':').collect();
            if parts.len() == 5 && parts[0] == "N" {
                if let (Ok(l), Ok(n), Ok(thresh), Ok(bias)) = (
                    parts[1].parse::<usize>(),
                    parts[2].parse::<usize>(),
                    parts[3].parse::<i32>(),
                    parts[4].parse::<i32>(),
                ) {
                    if let Some(layer) = self.layers.get_mut(l) {
                        if let Some(neuron) = layer.neurons.get_mut(n) {
                            neuron.threshold = thresh;
                            neuron.bias = bias;
                        }
                    }
                }
            } else if parts.len() == 5 && parts[0] == "W" {
                if let (Ok(l), Ok(n), Ok(w_idx), Ok(trit)) = (
                    parts[1].parse::<usize>(),
                    parts[2].parse::<usize>(),
                    parts[3].parse::<usize>(),
                    parts[4].parse::<i32>(),
                ) {
                    if let Some(layer) = self.layers.get_mut(l) {
                        if let Some(neuron) = layer.neurons.get_mut(n) {
                            if let Some(target_w) =
                                neuron.weights.get_mut(w_idx)
                            {
                                *target_w = SynapticWeight::from_trit(trit);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Maximum number of historical weight checkpoints retained in Track 4.
    pub const MAX_WEIGHT_CHECKPOINTS: usize = 5;

    /// Saves the current neural weights to Track 4 storage and maintains exactly the last 5 checkpoints.
    pub fn save_weights_to_track4(&self) -> Result<std::path::PathBuf, String> {
        let weights_dir = crate::core::filetype::resolve_weights_storage_dir();
        std::fs::create_dir_all(&weights_dir)
            .map_err(|e| format!("FAILPOINT [WeightsDirError]: Failed to create {weights_dir:?}: {e}"))?;

        // 1. Primary active weights file
        let primary_path = weights_dir.join("cpisi_base.weights");
        self.save_weights(&primary_path)?;

        // 2. Timestamped checkpoint
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        let checkpoint_name = format!("cpisi_base_{now}.weights");
        let checkpoint_path = weights_dir.join(&checkpoint_name);
        self.save_weights(&checkpoint_path)?;

        // Mirror to local relative path if distinct
        let local_path = std::path::PathBuf::from("storage/weights/cpisi_base.weights");
        if local_path != primary_path {
            if let Some(parent) = local_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = self.save_weights(&local_path);
        }

        // 3. Prune older checkpoints, keeping exactly the last 5
        Self::prune_checkpoints(&weights_dir, Self::MAX_WEIGHT_CHECKPOINTS).ok();

        Ok(primary_path)
    }

    /// Prunes checkpoint weight files in weights_dir, keeping the most recent `keep_count` checkpoints.
    pub fn prune_checkpoints(weights_dir: &std::path::Path, keep_count: usize) -> Result<usize, String> {
        let entries = std::fs::read_dir(weights_dir).map_err(|e| e.to_string())?;
        let mut checkpoints: Vec<(std::path::PathBuf, u128, std::time::SystemTime)> = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("cpisi_base_") && name.ends_with(".weights") && name != "cpisi_base.weights" {
                    let ts = name
                        .strip_prefix("cpisi_base_")
                        .and_then(|s| s.strip_suffix(".weights"))
                        .and_then(|s| s.parse::<u128>().ok())
                        .unwrap_or(0);
                    let mtime = entry.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    checkpoints.push((path, ts, mtime));
                }
            }
        }

        // Sort ascending by (timestamp, mtime) so oldest come first
        checkpoints.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2)));

        let mut removed = 0;
        if checkpoints.len() > keep_count {
            let to_remove = checkpoints.len() - keep_count;
            for (path, _, _) in checkpoints.iter().take(to_remove) {
                if std::fs::remove_file(path).is_ok() {
                    removed += 1;
                }
            }
        }

        Ok(removed)
    }

    /// Loads the latest weights from Track 4 if available.
    pub fn load_latest_weights_from_track4(&mut self) -> Result<std::path::PathBuf, String> {
        let weights_dir = crate::core::filetype::resolve_weights_storage_dir();
        let primary_path = weights_dir.join("cpisi_base.weights");
        if primary_path.exists() {
            self.load_weights(&primary_path)?;
            return Ok(primary_path);
        }

        // Fallback to local storage/weights/cpisi_base.weights
        let local_path = std::path::PathBuf::from("storage/weights/cpisi_base.weights");
        if local_path.exists() {
            self.load_weights(&local_path)?;
            return Ok(local_path);
        }

        // Fallback to most recent checkpoint
        let entries = std::fs::read_dir(&weights_dir).map_err(|e| e.to_string())?;
        let mut checkpoints: Vec<(std::path::PathBuf, u128, std::time::SystemTime)> = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("cpisi_base_") && name.ends_with(".weights") && name != "cpisi_base.weights" {
                    let ts = name
                        .strip_prefix("cpisi_base_")
                        .and_then(|s| s.strip_suffix(".weights"))
                        .and_then(|s| s.parse::<u128>().ok())
                        .unwrap_or(0);
                    let mtime = entry.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    checkpoints.push((path, ts, mtime));
                }
            }
        }

        checkpoints.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2)));
        if let Some((latest_path, _, _)) = checkpoints.last() {
            self.load_weights(latest_path)?;
            return Ok(latest_path.clone());
        }

        Err("FAILPOINT [NoWeightsFound]: No weight files found in Track 4 storage".to_string())
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
    fn test_brain_forward_and_adapt() {
        let logger = DiagnosticLogger::new(false);
        let mut brain = NeuralBrain::new(&[3, 4, 2], 1, logger);
        let out = brain.think(&[
            QuadState::Proceed,
            QuadState::Proceed,
            QuadState::Proceed,
        ]);
        assert_eq!(out.len(), 2);

        brain.adapt_synapse(0, 0, 0, QuadState::Proceed);
        let out2 = brain.think(&[
            QuadState::Proceed,
            QuadState::Proceed,
            QuadState::Proceed,
        ]);
        assert_eq!(out2.len(), 2);
    }

    #[test]
    fn test_brain_encode_stimulus_and_persistence() {
        let logger = DiagnosticLogger::new(false);
        let mut brain = NeuralBrain::new(&[8, 16, 4], 1, logger.clone());
        let stimulus = NeuralBrain::encode_stimulus("Hello", 8);
        assert_eq!(stimulus.len(), 8);

        let out = brain.think(&stimulus);
        assert_eq!(out.len(), 4);

        let (exc, neu, inh) = brain.synaptic_counts();
        assert!(exc + neu + inh > 0);

        let tmp_path = std::env::temp_dir().join("test_cpisi_brain.weights");
        brain.adapt_synapse(0, 0, 0, QuadState::Proceed);
        brain.save_weights(&tmp_path).unwrap();

        let mut loaded_brain = NeuralBrain::new(&[8, 16, 4], 1, logger);
        loaded_brain.load_weights(&tmp_path).unwrap();
        assert_eq!(
            loaded_brain.layers[0].neurons[0].weights[0],
            SynapticWeight::Excitatory
        );
        std::fs::remove_file(&tmp_path).ok();
    }

    #[test]
    fn test_track4_weight_checkpoints_rotation_keeps_last_five() {
        let test_dir = std::env::temp_dir().join(format!("omnicode_test_weights_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&test_dir);
        std::fs::create_dir_all(&test_dir).unwrap();

        let logger = DiagnosticLogger::new(false);
        let brain = NeuralBrain::new(&[4, 8, 2], 1, logger);

        // Create 8 timestamped checkpoints: cpisi_base_100.weights .. cpisi_base_800.weights
        for i in 1..=8 {
            let path = test_dir.join(format!("cpisi_base_{}.weights", i * 100));
            brain.save_weights(&path).unwrap();
        }

        // Also create the active un-suffixed file
        let primary = test_dir.join("cpisi_base.weights");
        brain.save_weights(&primary).unwrap();

        // Total checkpoint files before prune = 8 (excluding primary)
        let count_before = std::fs::read_dir(&test_dir).unwrap().count();
        assert_eq!(count_before, 9); // 8 checkpoints + 1 primary

        // Prune keeping exactly the last 5
        let removed = NeuralBrain::prune_checkpoints(&test_dir, 5).unwrap();
        assert_eq!(removed, 3); // 8 - 5 = 3 oldest checkpoints removed

        let mut remaining_checkpoints = Vec::new();
        for entry in std::fs::read_dir(&test_dir).unwrap().flatten() {
            let name = entry.file_name().into_string().unwrap();
            if name.starts_with("cpisi_base_") {
                remaining_checkpoints.push(name);
            }
        }

        assert_eq!(remaining_checkpoints.len(), 5);
        remaining_checkpoints.sort();
        // The remaining 5 MUST be the newest ones: 400, 500, 600, 700, 800
        assert_eq!(remaining_checkpoints, vec![
            "cpisi_base_400.weights",
            "cpisi_base_500.weights",
            "cpisi_base_600.weights",
            "cpisi_base_700.weights",
            "cpisi_base_800.weights",
        ]);

        // Primary active file remains untouched
        assert!(primary.exists());

        // Cleanup
        let _ = std::fs::remove_dir_all(&test_dir);
    }
}
