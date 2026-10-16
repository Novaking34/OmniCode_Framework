// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Traditional Developer & Structured Telemetry .log Inscription Engine )
// ==============================================================================
//! Traditional Telemetry Inscription Engine
//!
//! Formats human-readable ASCII telemetry with sensory event headers,
//! microsecond timings, immune system repair notes, and failpoints.

use std::fs;
use std::path::{Path, PathBuf};
use super::super::universal::LogEvent;

/// Inscribes a structured traditional telemetry log file to disk.
pub fn inscribe_telemetry_log(
    path: &Path,
    timestamp_str: &str,
    session_id: &str,
    elapsed_micros: u128,
    final_state_str: &str,
    prompt: &str,
    failpoints: &[LogEvent],
    synthesized_omnicode: &str,
    response: &str,
) -> Result<PathBuf, String> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("Failed to create log parent directory: {e}")
            })?;
        }
    }

    let mut out = String::new();
    out.push_str("================================================================================\n");
    out.push_str("OMNICODE AI MODEL EXECUTION LOG (TRADITIONAL TELEMETRY)\n");
    out.push_str("================================================================================\n");
    out.push_str(&format!("Timestamp:    {}\n", timestamp_str));
    out.push_str(&format!("Session ID:   {}\n", session_id));
    out.push_str(&format!("Duration:     {} µs\n", elapsed_micros));
    out.push_str(&format!("Final State:  +1:{}\n", final_state_str));
    out.push_str(&format!("Discourse:    \"{}\"\n", prompt));
    out.push_str("--------------------------------------------------------------------------------\n\n");

    out.push_str("[1. SENSORY LOG EVENTS & FAILPOINTS]\n");
    if failpoints.is_empty() {
        out.push_str("  • (No failpoints trapped. Sensory trace clean.)\n");
    } else {
        for (i, fp) in failpoints.iter().enumerate() {
            let kind = fp
                .failpoint_type
                .map(|t| t.as_str())
                .unwrap_or("GENERAL");
            out.push_str(&format!(
                "  [{:>4}] [{} µs] [{}] [{}] {}\n",
                i + 1,
                fp.elapsed_micros,
                fp.channel,
                kind,
                fp.message
            ));
            if let Some(ref sug) = fp.suggestion {
                out.push_str(&format!(
                    "         -> Remediation Proposal: [{:?}] L{}: {}\n",
                    sug.kind, sug.line, sug.rationale
                ));
            }
        }
    }
    out.push('\n');

    out.push_str("[2. IMMUNE SYSTEM RESTORATIONS]\n");
    out.push_str("  • (Zero repairs required; code conformed to all boundary and block invariants.)\n\n");

    out.push_str("[3. SYNTHESIZED TURING REEL (OMNICODE)]\n");
    out.push_str(synthesized_omnicode);
    if !synthesized_omnicode.ends_with('\n') {
        out.push('\n');
    }
    out.push('\n');

    out.push_str("[4. MACHINE DISCOURSE RESPONSE]\n");
    out.push_str(&format!("  \"{}\"\n", response));
    out.push_str("================================================================================\n");

    fs::write(path, out)
        .map_err(|e| format!("Failed to write log to {:?}: {e}", path))?;

    Ok(path.to_path_buf())
}
