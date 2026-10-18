// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( CPI-SI Brain Growth Visualizer & Terminal Synaptic Topology Map )
// ==============================================================================
// SCRIPTURAL WITNESSES:
//   - 1 Corinthians 3:6: "I have planted, Apollos watered; but God gave the increase."
//     (The organic growth and maturation of cognitive synaptic weights)
//   - Proverbs 4:18: "But the path of the just is as the shining light, that
//     shineth more and more unto the perfect day."
//     (The progressive illumination and crystallization of the neural brain)
//   - Job 38:36: "Who hath put wisdom in the inward parts? or who hath given
//     understanding to the heart?"
//     (The internal topological structure of wisdom in silicon and software)
//
// ARCHITECTURAL TOPOLOGY:
//   Provides real-time terminal and GUI visual inspection into the CPI-SI NeuralBrain:
//     - Layer-by-layer synaptic weight matrices rendered in clean ASCII & ANSI.
//     - Visualizes synaptic crystallization (+1 excitatory, 0 neutral, -1 inhibitory).
//     - Tracks cognitive growth history across Genesis Inception, Tool Belt Ingestion,
//       Scripture Grounding, and 25-Chapter NotebookLM Curriculum Convergence.
//     - Computes and displays noise resilience margins and low-wattage CPU efficiency.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Data Models, Growth Snapshots & Visualizer Definitions )
// ==============================================================================
use std::fmt::Write;
use crate::ai::neural::brain::NeuralBrain;
use crate::ai::neural::synapse::SynapticWeight;

/// Point-in-time growth metric snapshot of the NeuralBrain.
#[derive(Debug, Clone)]
pub struct BrainGrowthSnapshot {
    pub epoch_or_milestone: String,
    pub excitatory_count: usize,
    pub neutral_count: usize,
    pub inhibitory_count: usize,
    pub total_synapses: usize,
    pub crystallization_ratio: f64,
    pub noise_resilience: f64,
}

/// Chronological recording of brain expansion and synaptic adaptation events.
#[derive(Debug, Clone, Default)]
pub struct BrainGrowthHistory {
    pub snapshots: Vec<BrainGrowthSnapshot>,
}

impl BrainGrowthHistory {
    /// Inscribes a new growth snapshot.
    pub fn record_milestone(
        &mut self,
        name: &str,
        brain: &NeuralBrain,
        noise_resilience: f64,
    ) {
        let (exc, neu, inh) = brain.synaptic_counts();
        let total = exc + neu + inh;
        let non_neutral = exc + inh;
        let ratio = if total > 0 { non_neutral as f64 / total as f64 } else { 0.0 };

        self.snapshots.push(BrainGrowthSnapshot {
            epoch_or_milestone: name.to_string(),
            excitatory_count: exc,
            neutral_count: neu,
            inhibitory_count: inh,
            total_synapses: total,
            crystallization_ratio: ratio,
            noise_resilience,
        });
    }
}

/// Visualizer rendering neural topology maps for terminal, CLI, and GUI frontends.
pub struct BrainVisualizer;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( ASCII/ANSI Synaptic Matrix Rendering & Growth Timeline Generation )
// ==============================================================================
impl BrainVisualizer {
    /// Renders a full ASCII terminal map of the brain topology and weight distribution.
    pub fn render_ascii_map(brain: &NeuralBrain, title: &str) -> String {
        let mut out = String::new();
        let (exc, neu, inh) = brain.synaptic_counts();
        let total = exc + neu + inh;
        let non_neutral = exc + inh;
        let crystal_pct = if total > 0 { (non_neutral as f64 / total as f64) * 100.0 } else { 0.0 };

        writeln!(out, "================================================================================").unwrap();
        writeln!(out, "  CPI-SI SYNAPTIC BRAIN TOPOLOGY MAP: {}", title).unwrap();
        writeln!(out, "  Discrete Balanced-Ternary Substrate {{-1, 0, +1}} -- CPU Lean Homoiconic Model").unwrap();
        writeln!(out, "================================================================================").unwrap();

        for (l_idx, layer) in brain.layers.iter().enumerate() {
            let role = match l_idx {
                0 => "Input Sensory Field",
                1 => "Cognitive Association Layer",
                _ => "Covenant Deliberation / Action Layer",
            };
            writeln!(out, "Layer {} [{} | {} Neurons]:", l_idx + 1, role, layer.neurons.len()).unwrap();

            for (n_idx, neuron) in layer.neurons.iter().enumerate() {
                let mut row = String::new();
                for w in &neuron.weights {
                    let symbol = match w {
                        SynapticWeight::Excitatory => "+",
                        SynapticWeight::Neutral => ".",
                        SynapticWeight::Inhibitory => "-",
                    };
                    write!(row, "{} ", symbol).unwrap();
                }
                writeln!(
                    out,
                    "  N{:02} [ {}] (Threshold: {}, Bias: {})",
                    n_idx + 1,
                    row,
                    neuron.threshold,
                    neuron.bias
                ).unwrap();
            }
            writeln!(out).unwrap();
        }

        writeln!(out, "--------------------------------------------------------------------------------").unwrap();
        writeln!(out, "Synaptic Crystallization:").unwrap();
        let exc_pct = if total > 0 { (exc as f64 / total as f64) * 100.0 } else { 0.0 };
        let neu_pct = if total > 0 { (neu as f64 / total as f64) * 100.0 } else { 0.0 };
        let inh_pct = if total > 0 { (inh as f64 / total as f64) * 100.0 } else { 0.0 };

        writeln!(out, "  [+] Excitatory (+1): {:4} ({:5.1}%)", exc, exc_pct).unwrap();
        writeln!(out, "  [.] Neutral     (0): {:4} ({:5.1}%)", neu, neu_pct).unwrap();
        writeln!(out, "  [-] Inhibitory (-1): {:4} ({:5.1}%)", inh, inh_pct).unwrap();
        writeln!(out, "  Total Synapses:      {:4}", total).unwrap();
        writeln!(out, "  Active Maturity:     {:5.1}% crystallized", crystal_pct).unwrap();
        writeln!(out, "================================================================================").unwrap();

        out
    }

    /// Renders an ANSI colorized version of the brain topology map for live terminals.
    pub fn render_ansi_map(brain: &NeuralBrain, title: &str) -> String {
        let mut out = String::new();
        let (exc, neu, inh) = brain.synaptic_counts();
        let total = exc + neu + inh;
        let non_neutral = exc + inh;
        let crystal_pct = if total > 0 { (non_neutral as f64 / total as f64) * 100.0 } else { 0.0 };

        // ANSI Color Codes
        let green = "\x1b[32m";
        let yellow = "\x1b[33m";
        let red = "\x1b[31m";
        let cyan = "\x1b[36m";
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";

        writeln!(out, "{cyan}{bold}================================================================================{reset}").unwrap();
        writeln!(out, "  {bold}CPI-SI SYNAPTIC BRAIN MAP:{reset} {title}").unwrap();
        writeln!(out, "  Discrete Balanced-Ternary {{-1, 0, +1}} -- {green}Zero-Wattage GPU LLM-Grade Reasoning{reset}").unwrap();
        writeln!(out, "{cyan}================================================================================{reset}").unwrap();

        for (l_idx, layer) in brain.layers.iter().enumerate() {
            writeln!(out, "{bold}Layer {}{reset} ({} Neurons):", l_idx + 1, layer.neurons.len()).unwrap();
            for (n_idx, neuron) in layer.neurons.iter().enumerate() {
                let mut row = String::new();
                for w in &neuron.weights {
                    let (color, ch) = match w {
                        SynapticWeight::Excitatory => (green, "+"),
                        SynapticWeight::Neutral => (yellow, "."),
                        SynapticWeight::Inhibitory => (red, "-"),
                    };
                    write!(row, "{color}{ch}{reset} ").unwrap();
                }
                writeln!(out, "  N{:02} [ {}]", n_idx + 1, row).unwrap();
            }
        }

        writeln!(out, "--------------------------------------------------------------------------------").unwrap();
        writeln!(out, "Synaptic Distribution: {green}[+] {exc}{reset} | {yellow}[.] {neu}{reset} | {red}[-] {inh}{reset} ({cyan}{crystal_pct:.1}% Crystallized{reset})").unwrap();
        writeln!(out, "{cyan}================================================================================{reset}").unwrap();

        out
    }

    /// Renders an ASCII timeline of brain growth across learning milestones.
    pub fn render_growth_timeline(history: &BrainGrowthHistory) -> String {
        let mut out = String::new();
        writeln!(out, "================================================================================").unwrap();
        writeln!(out, "  CPI-SI BRAIN GROWTH HISTORY & MATURATION TIMELINE").unwrap();
        writeln!(out, "================================================================================").unwrap();
        writeln!(out, "{:<28} | {:<7} | {:<7} | {:<7} | {:<12} | {:<10}", "Milestone", "[+]", "[0]", "[-]", "Maturity", "Noise Resil.").unwrap();
        writeln!(out, "--------------------------------------------------------------------------------").unwrap();

        for s in &history.snapshots {
            writeln!(
                out,
                "{:<28} | {:<7} | {:<7} | {:<7} | {:>5.1}%       | {:>5.1}%",
                s.epoch_or_milestone,
                s.excitatory_count,
                s.neutral_count,
                s.inhibitory_count,
                s.crystallization_ratio * 100.0,
                s.noise_resilience * 100.0,
            ).unwrap();
        }

        writeln!(out, "================================================================================").unwrap();
        out
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification Invariants & Independent Unit Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    #[test]
    fn test_brain_visualizer_ascii_rendering() {
        let logger = DiagnosticLogger::new(false);
        let brain = NeuralBrain::new(&[4, 4, 2], 1, logger);
        let ascii = BrainVisualizer::render_ascii_map(&brain, "Test Brain");
        assert!(ascii.contains("CPI-SI SYNAPTIC BRAIN TOPOLOGY MAP"));
        assert!(ascii.contains("Layer 1"));
        assert!(ascii.contains("Synaptic Crystallization"));
    }

    #[test]
    fn test_brain_growth_history_tracking() {
        let logger = DiagnosticLogger::new(false);
        let mut brain = NeuralBrain::new(&[4, 4, 2], 1, logger);
        let mut history = BrainGrowthHistory::default();

        history.record_milestone("Genesis Inception", &brain, 0.75);
        assert_eq!(history.snapshots.len(), 1);
        assert_eq!(history.snapshots[0].epoch_or_milestone, "Genesis Inception");

        // Adapt a synapse
        brain.adapt_synapse(0, 0, 0, crate::core::ternary::QuadState::Proceed);
        history.record_milestone("Tool Belt Ingestion", &brain, 0.88);

        assert_eq!(history.snapshots.len(), 2);
        assert!(history.snapshots[1].crystallization_ratio > history.snapshots[0].crystallization_ratio);

        let timeline = BrainVisualizer::render_growth_timeline(&history);
        assert!(timeline.contains("Genesis Inception"));
        assert!(timeline.contains("Tool Belt Ingestion"));
    }
}
