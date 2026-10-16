// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Canonical 4-Block OmniCode .scroll Inscription Engine )
// ==============================================================================
//! Canonical Scroll Inscription Engine
//!
//! Formats and physically inscribes executable 4-Block OmniCode scrolls.

use std::fs;
use std::path::{Path, PathBuf};
use super::super::telemetry::CognitiveNote;

/// Inscribes a canonical 4-Block .scroll file to the target path.
pub fn inscribe_canonical_scroll(
    path: &Path,
    session_id: &str,
    prompt: &str,
    synthesized_omnicode: &str,
    state_str: &str,
    response: &str,
    elapsed_micros: u128,
) -> Result<PathBuf, String> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("Failed to create scroll parent directory: {e}")
            })?;
        }
    }

    let indented_reel = synthesized_omnicode
        .lines()
        .map(|l| format!("        {l}"))
        .collect::<Vec<_>>()
        .join("\n");

    let scroll_content = format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ( Execution Trace ID, Prompt Intent, Inferred Types, and Neural Snapshot )
# ==============================================================================
Scroll < EpistemicTrace.{session_id} > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    session_id: < "{session_id}" >,
    execution_duration_micros: < {elapsed_micros} >,
    cognitive_mode: < NeuralSymbolicTransduction >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ( Hardware Invariants, Preconditions, and Initial Context )
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}}

user_prompt < "{prompt}" >

# ==============================================================================
# BLOCK 3: BODY BLOCK (CHRONICLE OF EXECUTION & THOUGHT REEL)
# ( Replay Tape: Synthesized Program, Sensory Telemetry & Immune Interventions )
# ==============================================================================
Body [
    # Executed Turing Reel Synthesized by AI Model:
    synthesized_reel [
{indented_reel}
    ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (EPISODIC COMMIT & LEARNING PROOF)
# ( Verification Proof, Final Quad-State, and Conversational Synthesis )
# ==============================================================================
Closing {{
    commit_status [ <{state_str}> ],
    machine_utterance ( "{sanitized_response}" )
}}
"#,
        session_id = session_id,
        prompt = prompt,
        elapsed_micros = elapsed_micros,
        indented_reel = indented_reel,
        state_str = state_str,
        sanitized_response = response.replace('"', "\\\"")
    );

    fs::write(path, scroll_content)
        .map_err(|e| format!("Failed to write scroll to {:?}: {e}", path))?;

    Ok(path.to_path_buf())
}

/// Inscribes a complete cognitive audit scroll containing multiple notes.
pub fn inscribe_audit_scroll(
    notes: &[CognitiveNote],
    failpoints_count: usize,
) -> String {
    let mut out = String::new();
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 1: METADATA BLOCK\n");
    out.push_str("# ( Episodic AI Cognitive Memory Scroll & Incident Audit Log )\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Scroll < CognitiveMemory.Audit > {\n");
    out.push_str(&format!("    total_notes: < {} >,\n", notes.len()));
    out.push_str(&format!("    total_failpoints: < {} >\n", failpoints_count));
    out.push_str("}\n\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 2: OPENING BLOCK\n");
    out.push_str("# ( Cognitive Context & Invariant Preconditions )\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Opening {\n    preconditions: [ +1 ]\n}\n\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 3: BODY BLOCK\n");
    out.push_str("# ( Episodic Traces & Memory Units Recorded During Execution )\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Body [\n");
    for n in notes {
        out.push_str(&format!(
            "    {}\n\n",
            n.format_as_omnicode().replace('\n', "\n    ")
        ));
    }
    out.push_str("]\n\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 4: CLOSING BLOCK\n");
    out.push_str("# ( Cognitive Audit Verification Proof )\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Closing {\n    audit_verified: [ PROCEED ]\n}\n");
    out
}
