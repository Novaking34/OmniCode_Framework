// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Interactive Session State, Direct Roll Evaluator & Native Echo Engine )
// ==============================================================================
//! OmniCode Terminal: Interactive Session Coordinator & Character I/O Engine
//!
//! # Architectural Scope & Purpose:
//! Coordinates the interactive userspace shell for the OmniCode platform:
//! 1. Maintains command history, variable declarations, and persistent [`Environment`] state.
//! 2. Routes shell commands through the Multi-Track [`OmniScheduler`].
//! 3. Directly evaluates live procedural rolls in memory without sub-process overhead.
//! 4. Houses native Character I/O streams and the 5-trit Radix-3 [`CharacterEchoEngine`].
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of atomic runes onto bounded physical tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
// ==============================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::cogs::tripwire::Tripwire;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;
use crate::terminal::scheduler::{OmniScheduler, SharedScheduler};
use crate::terminal::universal::call_response_engine::{
    Call, Response, CharacterResponse, NumberResponse, CallResponseEngine, EchoEngine,
};
use crate::terminal::universal::ports::{TerminalInputStream, TerminalOutputStream};

use super::commands::{dispatch_builtin_command, ExecutionResult};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( TerminalSession Struct Definition & Initializers )
// ==============================================================================

/// Interactive shell session maintaining persistent memory bindings,
/// scheduler access, directional I/O streams, and native character echo.
pub struct TerminalSession {
    /// Command line history.
    pub history: Vec<String>,
    /// Script-level variable bindings.
    pub variables: HashMap<String, String>,
    /// Thread-safe handle to the cooperative Multi-Track scheduler.
    pub scheduler: SharedScheduler,
    /// Persistent virtual machine runtime environment.
    pub env: Environment,
    /// Attached telemetry and error interception logger.
    pub logger: DiagnosticLogger,
    /// Delimiter boundary stack tracking debugger.
    pub debugger: OmniDebugger,
    /// Directional input stream from hardware `FROM` port (0x03F8).
    pub input_stream: TerminalInputStream,
    /// Directional output stream to hardware `INTO` port (0x000B8000).
    pub output_stream: TerminalOutputStream,
    /// Universal Call-and-Response Echo Engine.
    pub echo_engine: EchoEngine,
}

impl Default for TerminalSession {
    fn default() -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let env = Environment::new();
        Self {
            history: Vec::new(),
            variables: HashMap::new(),
            scheduler: Arc::new(Mutex::new(OmniScheduler::new())),
            env,
            logger,
            debugger,
            input_stream: TerminalInputStream::default(),
            output_stream: TerminalOutputStream::default(),
            echo_engine: EchoEngine,
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Execution Pipeline, Direct Roll Lowering & Native Character I/O )
// ==============================================================================

impl TerminalSession {
    /// Constructs a new terminal session bound to a shared multi-track scheduler.
    pub fn new(scheduler: SharedScheduler) -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let env = Environment::new();
        Self {
            history: Vec::new(),
            variables: HashMap::new(),
            scheduler,
            env,
            logger,
            debugger,
            input_stream: TerminalInputStream::default(),
            output_stream: TerminalOutputStream::default(),
            echo_engine: EchoEngine,
        }
    }

    /// Dispatches a command line and returns an [`ExecutionResult`].
    pub fn execute(&mut self, line: &str) -> ExecutionResult {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return ExecutionResult::success(String::new(), 0);
        }

        self.history.push(trimmed.to_string());
        let start = Instant::now();

        // 1. Dispatch Built-in Shell Commands
        if let Some(res) = dispatch_builtin_command(
            trimmed,
            &self.scheduler,
            &self.env,
            &mut self.variables,
            &self.logger,
            &self.debugger,
            start,
        ) {
            for c in res.stdout.chars() {
                self.output_stream.push_char(c);
            }
            return res;
        }

        // 2. Direct In-Memory Roll Evaluation (OmniCode Native Execution)
        if trimmed.contains("-->") || trimmed.contains("<--") || trimmed.contains("v--") || trimmed.contains("--^")
            || trimmed.starts_with("Storehouse") || (trimmed.contains('[') && trimmed.contains(']'))
            || (trimmed.contains('<') && trimmed.contains('>'))
            || trimmed.starts_with("status")
        {
            let mut lexer = Lexer::new(trimmed, self.logger.clone());
            if let Ok(tokens) = lexer.tokenize() {
                let mut parser = Parser::new(tokens, self.debugger.clone());
                if let Ok(prog) = parser.parse_program() {
                    let mut evaluator = Evaluator::new(self.env.clone(), self.debugger.clone());
                    match evaluator.eval(&crate::ast::nodes::ASTNode::Program(prog)) {
                        Ok(val) => {
                            self.env = evaluator.env; // Persist environment state!
                            return ExecutionResult::success(
                                format!("=> {:?} (state: +1:PROCEED)", val),
                                start.elapsed().as_micros(),
                            );
                        }
                        Err(e) => {
                            let err = Tripwire::trip(
                                &self.logger,
                                "TERMINAL_EVAL",
                                FailpointType::SyntaxAnomaly,
                                format!("Evaluation Failpoint: {e}"),
                            );
                            return ExecutionResult::failure(
                                format!("[!] {err}"),
                                start.elapsed().as_micros(),
                            );
                        }
                    }
                }
            }
        }

        // 3. Fallback: Host Subcommand Execution
        let clean_cmd = if trimmed.starts_with("omnicode ") {
            trimmed.to_string()
        } else {
            format!("omnicode {trimmed}")
        };

        let exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("omnicode"));
        let args: Vec<&str> = clean_cmd["omnicode ".len()..].split_whitespace().collect();
        let output = std::process::Command::new(&exe)
            .args(&args)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let combined = if stderr.is_empty() {
                    stdout
                } else if stdout.is_empty() {
                    stderr
                } else {
                    format!("{stdout}\n{stderr}")
                };
                if out.status.success() {
                    ExecutionResult::success(combined, start.elapsed().as_micros())
                } else {
                    ExecutionResult::failure(combined, start.elapsed().as_micros())
                }
            }
            Err(e) => ExecutionResult::failure(
                format!("Failed to execute CLI subcommand: {e}"),
                start.elapsed().as_micros(),
            ),
        }
    }

    // --------------------------------------------------------------------------
    // Native Character I/O & Echo Stream Methods
    // --------------------------------------------------------------------------

    /// Feeds an input character into the terminal's `FROM` port stream (0x03F8)
    /// and synchronizes it into the virtual machine environment.
    pub fn feed_input_char(&mut self, c: char) {
        self.input_stream.push_char(c);
        self.env.write_from(RuntimeValue::from_char(c));
    }

    /// Reads the next available character from the terminal's `INTO` port stream (0x000B8000).
    pub fn read_output_char(&mut self) -> Option<char> {
        self.output_stream.pop_char()
    }

    /// Flushes all pending output characters into an allocated string.
    pub fn flush_output(&mut self) -> String {
        self.output_stream.flush_to_string()
    }

    /// Processes a single character glyph through the full 5-step Character tape lifecycle,
    /// verifying Radix-3 transduction, single-cell capacity bounds, and returning the native echo.
    pub fn process_character_echo(&mut self, glyph: char) -> Result<CharacterResponse, String> {
        // Feed into FROM port
        self.feed_input_char(glyph);

        // Run full Inscription -> Transduction -> Bound Guard -> Target Projection
        let res = EchoEngine::echo_char(glyph)?;

        // Write projected character into INTO port and output stream
        self.output_stream.push_char(res.glyph);
        self.env.write_into(RuntimeValue::from_char(res.glyph));

        // Inscribe RollFrame telemetry into in-memory flight recorder
        let frame = crate::diagnostics::logger::RollFrame::new(
            res.glyph as u128,
            1,
            5,
            "TRANSDUCE",
            (0, res.glyph as u64),
            (res.glyph as u64, 0),
            res.state,
            0,
            Some(format!("Glyph '{}'", res.glyph)),
        );
        self.logger.record_roll_frame(frame);

        Ok(res)
    }

    /// Processes an entire character stream through the native Character Echo protocol,
    /// returning the verified echoed string.
    pub fn process_character_stream_echo(&mut self, stream: &str) -> Result<String, String> {
        let mut echoed = String::with_capacity(stream.len());
        for c in stream.chars() {
            let res = self.process_character_echo(c)?;
            echoed.push(res.glyph);
        }
        Ok(echoed)
    }

    /// Processes any Call payload through the universal Call-Response model.
    pub fn process_call(&mut self, call: Call) -> Result<Response, String> {
        CallResponseEngine::execute(call)
    }

    /// Processes a scalar Number through the Call-Response model.
    pub fn process_number_call(&mut self, scalar: i64) -> Result<NumberResponse, String> {
        let res = CallResponseEngine::call_number(scalar)?;
        self.env.write_into(RuntimeValue::from_number(scalar as f64));

        let frame = crate::diagnostics::logger::RollFrame::new(
            scalar.unsigned_abs() as u128,
            1,
            3,
            "TRANSDUCE",
            (0, scalar.unsigned_abs()),
            (res.packed_dual_rail, scalar.unsigned_abs()),
            QuadState::Proceed,
            0,
            Some(format!("Number {scalar}")),
        );
        self.logger.record_roll_frame(frame);

        Ok(res)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_terminal_session_builtin_commands() {
        let mut session = TerminalSession::default();
        let res_help = session.execute("help");
        assert_eq!(res_help.state, QuadState::Proceed);
        assert!(res_help.stdout.contains("OMNICODE NATIVE OPERATING SYSTEM SHELL"));

        let res_tasks = session.execute("tasks");
        assert_eq!(res_tasks.state, QuadState::Proceed);
        assert!(res_tasks.stdout.contains("kernel --daemon"));

        let res_tracks = session.execute("tracks");
        assert_eq!(res_tracks.state, QuadState::Proceed);
        assert!(res_tracks.stdout.contains("CoreDaemon"));
    }

    #[test]
    fn test_terminal_session_echo_uncommon_letters() {
        let mut session = TerminalSession::default();

        // 1. Echo uncommon uppercase letter 'Q' (ASCII 81)
        let res_q = session.execute("echo Q");
        assert_eq!(res_q.state, QuadState::Proceed);
        assert_eq!(res_q.stdout, "Q");
        assert_eq!(session.read_output_char(), Some('Q'));

        // 2. Echo uncommon letter 'Z' (ASCII 90)
        let res_z = session.execute("echo 'Z'");
        assert_eq!(res_z.state, QuadState::Proceed);
        assert_eq!(res_z.stdout, "Z");
        assert_eq!(session.read_output_char(), Some('Z'));

        // 3. Echo symbol '~' (ASCII 126)
        let res_tilde = session.execute("echo ~");
        assert_eq!(res_tilde.state, QuadState::Proceed);
        assert_eq!(res_tilde.stdout, "~");
        assert_eq!(session.read_output_char(), Some('~'));

        // 4. Echo string with mixed uncommon characters "QxZ~"
        let res_multi = session.execute("echo QxZ~");
        assert_eq!(res_multi.state, QuadState::Proceed);
        assert_eq!(res_multi.stdout, "QxZ~");
        let drained: String = std::iter::from_fn(|| session.read_output_char()).collect();
        assert_eq!(drained, "QxZ~");
    }

    #[test]
    fn test_terminal_session_run_character_tape_and_omni() {
        let mut session = TerminalSession::default();

        // 1. Run character process tape
        let res_tape = session.execute("run lexicon/01_nouns/ideas/types/character.tape");
        assert_eq!(res_tape.state, QuadState::Proceed);
        assert!(res_tape.stdout.contains("Mounted Tape"));
        assert!(res_tape.stdout.contains("Roll 1/6"));

        // 2. Run character 4-Block source reel
        let res_omni = session.execute("run lexicon/01_nouns/ideas/types/character.omni");
        assert_eq!(res_omni.state, QuadState::Proceed);
        assert!(res_omni.stdout.contains("OmniCode Source Reel Executed Successfully"));
    }

    #[test]
    fn test_terminal_session_direct_roll_evaluation() {
        let mut session = TerminalSession::default();
        let res = session.execute("roll_1 [ A = A ]");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.stdout.contains("+1:PROCEED"));
    }

    #[test]
    fn test_terminal_session_native_character_echo() {
        let mut session = TerminalSession::default();
        let res = session.process_character_echo('K').expect("Native echo for 'K' must succeed");
        assert_eq!(res.glyph, 'K');
        assert_eq!(res.state, QuadState::Proceed);
        assert_eq!(res.cell_footprint, 1);
        assert_eq!(session.read_output_char(), Some('K'));
    }

    #[test]
    fn test_terminal_session_stream_echo() {
        let mut session = TerminalSession::default();
        let text = "OmniCode Native Terminal 2026!";
        let echoed = session.process_character_stream_echo(text).expect("Stream echo must succeed");
        assert_eq!(echoed, text);
    }

    #[test]
    fn test_terminal_exhaustive_character_echo_no_valid_character_omitted() {
        let mut session = TerminalSession::default();

        // 1. Test every standard ASCII character (0 through 127)
        // Including control characters (\0, \t, \n, \r), printable symbols, letters, and numbers
        let mut omitted_count = 0;
        let mut tested_count = 0;

        for byte in 0u8..=127u8 {
            let ch = byte as char;
            tested_count += 1;

            match session.process_character_echo(ch) {
                Ok(res) => {
                    assert_eq!(res.glyph, ch, "Identity violation for character code {byte}");
                    assert_eq!(res.state, QuadState::Proceed, "State violation for character code {byte}");
                    assert_eq!(res.cell_footprint, 1, "Cell footprint must be 1 for character code {byte}");

                    // Read from INTO port output stream
                    let read_back = session.read_output_char();
                    assert_eq!(read_back, Some(ch), "Output port character mismatch for {byte}");
                }
                Err(err) => {
                    eprintln!("Omission error for character {byte}: {err}");
                    omitted_count += 1;
                }
            }
        }

        assert_eq!(omitted_count, 0, "No valid ASCII character should be omitted");
        assert_eq!(tested_count, 128, "All 128 standard ASCII characters must be tested");

        // 2. Test extended single-cell character capacity up to 242 (3^5 = 243 discrete values)
        for byte in 128u8..=242u8 {
            let ch = byte as char;
            let res = session.process_character_echo(ch)
                .unwrap_or_else(|e| panic!("Failed echo for extended character {byte}: {e}"));
            assert_eq!(res.glyph, ch);
            assert_eq!(session.read_output_char(), Some(ch));
        }

        // 3. Test comprehensive printable ASCII stream
        let all_printable: String = (32u8..=126u8).map(|b| b as char).collect();
        let stream_echoed = session.process_character_stream_echo(&all_printable)
            .expect("Stream echo for all printable characters must succeed");
        assert_eq!(stream_echoed, all_printable, "Every single printable character must match identically without omission");
    }

    #[test]
    fn test_terminal_session_call_response_number() {
        let mut session = TerminalSession::default();

        // 1. Process number call for unities and partition
        let r0 = session.process_number_call(0).expect("Call 0");
        assert_eq!(r0.scalar, 0);
        assert!(r0.equilibrium_proved);

        let r1 = session.process_number_call(1).expect("Call 1");
        assert_eq!(r1.scalar, 1);

        let r_neg1 = session.process_number_call(-1).expect("Call -1");
        assert_eq!(r_neg1.scalar, -1);

        let r4096 = session.process_number_call(4096).expect("Call 4096");
        assert_eq!(r4096.scalar, 4096);
        assert_eq!(r4096.state, QuadState::Proceed);

        // 2. Process universal call
        let u_call = session.process_call(Call::Number(65536)).expect("Universal call");
        match u_call {
            Response::Number(n) => assert_eq!(n.scalar, 65536),
            _ => panic!("Expected Number response"),
        }
    }
}
