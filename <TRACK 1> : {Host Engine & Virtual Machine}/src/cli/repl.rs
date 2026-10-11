// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Interactive Terminal Environment Identity, REPL Architecture & Dependencies )
// ==============================================================================
//! OmniCode Terminal REPL: Interactive Ontological Shell
//!
//! Provides an interactive read-eval-print loop (REPL) enabling direct,
//! zero-latency experimentation with OmniCode's foundational primitives:
//!     - Balanced Ternary Logic (-1 Rollback, 0 Await, +1 Proceed, NULL Halt)
//!     - 6 Boundary Morphologies & Asymmetric Closures
//!     - First-Class Intent Asides (...)
//!     - 3D Coordinate Operators & Directional Vectors
//!     - Transactional Environment Snapshot & Rollback
//!     - Real-Time Diagnostic Failpoint Trapping

use std::fs;
use std::path::PathBuf;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::ast::nodes::ASTNode;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::linter::OmniLinter;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( REPL Configuration, Banner Display, and Context State Initialization )
// ==============================================================================
/// Resolves the file path for persistent terminal command history.
fn get_history_path() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".omnicode_history"))
}

/// Checks whether an interactive input buffer has balanced delimiters.
fn is_input_complete(input: &str) -> bool {
    let trimmed = input.trim_end();
    if trimmed.ends_with('\\') {
        return false;
    }

    let mut open_brackets = 0isize;
    let mut in_string = false;
    let mut quote_char = ' ';

    for line in input.lines() {
        let stripped = line.trim();
        if stripped.starts_with('#') {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if in_string {
                if c == '\\' {
                    i += 2;
                    continue;
                }
                if c == quote_char {
                    in_string = false;
                }
            } else {
                match c {
                    '#' => break, // Comment extends to end of line
                    '"' | '\'' => {
                        in_string = true;
                        quote_char = c;
                    }
                    '[' | '{' | '<' | '(' => open_brackets += 1,
                    ']' | '}' | '>' | ')' => open_brackets -= 1,
                    _ => {}
                }
            }
            i += 1;
        }
    }

    open_brackets <= 0
}

/// Displays the welcome banner and prompt instructions.
fn print_welcome_banner() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode Interactive Shell (v0.1.0)\x1b[0m");
    println!("\x1b[1;36m  Core: Balanced Ternary & Boundary Physics (Rust Engine)\x1b[0m");
    println!("\x1b[0;37m  Type \x1b[1;33m:help\x1b[0m\x1b[0;37m or \x1b[1;33m:?\x1b[0m\x1b[0;37m for commands & syntax cheat sheet. Press \x1b[1;33mCtrl+D\x1b[0m\x1b[0;37m to exit.\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");
}

/// Displays the help guide and syntax reference.
fn print_help_guide() {
    println!("\x1b[1;33m--- OmniCode Interactive Commands ---\x1b[0m");
    println!("  \x1b[1;32m:help\x1b[0m, \x1b[1;32m:?\x1b[0m               Show this help reference");
    println!("  \x1b[1;32m:env\x1b[0m                   Inspect all active symbols and entity bindings");
    println!("  \x1b[1;32m:ast <expr>\x1b[0m            Display raw AST structure of an expression");
    println!("  \x1b[1;32m:load <path>\x1b[0m           Load, lint, parse, and evaluate an OmniCode file (.omni / .scroll)");
    println!("  \x1b[1;32m:lint <path>\x1b[0m           Validate file against 4-Block architecture (.omni / .scroll)");
    println!("  \x1b[1;32m:failpoints\x1b[0m            Review all failpoint incidents trapped in this session");
    println!("  \x1b[1;32m:verbose [on|off]\x1b[0m      Toggle real-time microsecond diagnostic logging");
    println!("  \x1b[1;32m:clear\x1b[0m                 Clear the terminal screen");
    println!("  \x1b[1;32m:reset\x1b[0m                 Reset the environment and diagnostic stack");
    println!("  \x1b[1;32m:exit\x1b[0m, \x1b[1;32m:quit\x1b[0m           Exit the OmniCode interactive shell\n");

    println!("\x1b[1;33m--- Notation & Physics Cheat Sheet ---\x1b[0m");
    println!("  \x1b[1;36mBoundaries:\x1b[0m   [...] mutable | <...> immutable axiom | {{...}} context");
    println!("                {{...] top-down asymmetric | [...}} bottom-up asymmetric | (...) intent aside");
    println!("  \x1b[1;36mTernary:\x1b[0m      +1 (Proceed) | 0 (Await) | -1 (Rollback) | NULL (Halt)");
    println!("  \x1b[1;36mRelations:\x1b[0m    = (Is Identity) | != (Is Not) | ~= (To Be Like)");
    println!("  \x1b[1;36mVectors:\x1b[0m      --> (Act Upon / Forward) | <-- (Act Again / Rollback)");
    println!("                ^-- (Builds To) | v-- (Breaks Down From)");
    println!("  \x1b[1;36mConnectives:\x1b[0m  ! (Adversative Caveat BUT) | :: (Conjunction AND) | | (Disjunction OR)\n");
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core REPL Execution Loop: Command Dispatch, Parser, Evaluator Pipeline )
// ==============================================================================
/// Launches and executes the OmniCode interactive REPL terminal session.
pub fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    print_welcome_banner();

    let mut rl = DefaultEditor::new()?;
    let history_file = get_history_path();
    if let Some(ref path) = history_file {
        let _ = rl.load_history(path);
    }

    let logger = DiagnosticLogger::new(false);
    let mut debugger = OmniDebugger::new(logger.clone(), 10);
    let mut env = Environment::new();
    let mut current_state = QuadState::Proceed;
    let mut verbose = false;
    let mut multiline_buffer = String::new();

    loop {
        let prompt = if multiline_buffer.is_empty() {
            match current_state {
                QuadState::Proceed => "\x1b[1;32momni[+1]>\x1b[0m ".to_string(),
                QuadState::Await => "\x1b[1;33momni[0]>\x1b[0m ".to_string(),
                QuadState::Rollback => "\x1b[1;31momni[-1]>\x1b[0m ".to_string(),
                QuadState::Halt => "\x1b[1;35momni[NULL]>\x1b[0m ".to_string(),
            }
        } else {
            "\x1b[1;30m   ...>\x1b[0m ".to_string()
        };

        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let trimmed = line.trim();

                // Multiline buffering logic
                if !multiline_buffer.is_empty() {
                    if trimmed.is_empty() {
                        // Empty line forces execution of buffered input
                    } else {
                        multiline_buffer.push('\n');
                        multiline_buffer.push_str(&line);
                        if !is_input_complete(&multiline_buffer) {
                            continue;
                        }
                    }
                } else if !trimmed.is_empty() && !trimmed.starts_with(':') && !is_input_complete(trimmed) {
                    multiline_buffer.push_str(&line);
                    continue;
                }

                let input = if !multiline_buffer.is_empty() {
                    let full = multiline_buffer.clone();
                    multiline_buffer.clear();
                    full
                } else {
                    line
                };

                let input_trimmed = input.trim();
                if input_trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input_trimmed);

                // Command Dispatch
                if input_trimmed.starts_with(':') || input_trimmed == "exit" || input_trimmed == "quit" {
                    let cmd_parts: Vec<&str> = input_trimmed.splitn(2, ' ').collect();
                    let cmd = cmd_parts[0];
                    let arg = if cmd_parts.len() > 1 { cmd_parts[1].trim() } else { "" };

                    match cmd {
                        ":help" | ":?" => print_help_guide(),
                        ":exit" | ":quit" | "exit" | "quit" => {
                            println!("\x1b[1;36mSealing OmniCode session. Goodbye.\x1b[0m");
                            break;
                        }
                        ":clear" => {
                            print!("\x1b[2J\x1b[1;1H");
                        }
                        ":reset" => {
                            env.clear();
                            logger.clear();
                            debugger = OmniDebugger::new(logger.clone(), 10);
                            current_state = QuadState::Proceed;
                            println!("\x1b[1;32m✓ Session reset: Environment and failpoints cleared.\x1b[0m");
                        }
                        ":env" => {
                            let bindings = env.bindings();
                            if bindings.is_empty() {
                                println!("\x1b[0;37m(Environment is empty)\x1b[0m");
                            } else {
                                println!("\x1b[1;33m--- Active Symbols & Bindings ({}) ---\x1b[0m", bindings.len());
                                for (k, v) in bindings {
                                    println!("  \x1b[1;36m{}\x1b[0m = {}", k, v.format_colored());
                                }
                            }
                        }
                        ":tokens" => {
                            if arg.is_empty() {
                                eprintln!("\x1b[1;31mUsage:\x1b[0m :tokens <expression>");
                            } else {
                                let mut lex = Lexer::new(arg, logger.clone());
                                match lex.tokenize() {
                                    Ok(toks) => {
                                        println!("\x1b[1;33mTokens ({}):\x1b[0m", toks.len());
                                        for t in toks {
                                            println!("  {:?}  \x1b[0;37m('{}' at L{}:C{})\x1b[0m", t.token_type, t.lexeme, t.line, t.col);
                                        }
                                    }
                                    Err(e) => eprintln!("\x1b[1;31m✗ Lexer Error:\x1b[0m {}", e),
                                }
                            }
                        }
                        ":ast" => {
                            if arg.is_empty() {
                                eprintln!("\x1b[1;31mUsage:\x1b[0m :ast <expression>");
                            } else {
                                let mut lex = Lexer::new(arg, logger.clone());
                                match lex.tokenize() {
                                    Ok(toks) => {
                                        let mut p = Parser::new(toks, debugger.clone());
                                        match p.parse_program() {
                                            Ok(ast) => println!("{:#?}", ast),
                                            Err(e) => eprintln!("\x1b[1;31m✗ Parser Error:\x1b[0m {}", e),
                                        }
                                    }
                                    Err(e) => eprintln!("\x1b[1;31m✗ Lexer Error:\x1b[0m {}", e),
                                }
                            }
                        }
                        ":failpoints" => {
                            let fps = logger.failpoints();
                            if fps.is_empty() {
                                println!("\x1b[1;32m✓ No failpoint incidents recorded.\x1b[0m");
                            } else {
                                println!("\x1b[1;33m--- Trapped Failpoint Incidents ({}) ---\x1b[0m", fps.len());
                                for fp in fps {
                                    let fp_name = fp.failpoint_type.map(|t| t.as_str()).unwrap_or("GENERIC");
                                    println!("  * [{} µs] \x1b[1;31m[{}]\x1b[0m [{}] {}", fp.elapsed_micros, fp.channel, fp_name, fp.message);
                                }
                            }
                        }
                        ":verbose" => {
                            if arg == "on" {
                                verbose = true;
                                println!("\x1b[1;32m✓ Diagnostic verbose logging ENABLED.\x1b[0m");
                            } else if arg == "off" {
                                verbose = false;
                                println!("\x1b[1;33m✓ Diagnostic verbose logging DISABLED.\x1b[0m");
                            } else {
                                verbose = !verbose;
                                println!("Diagnostic verbose logging is now: \x1b[1;36m{}\x1b[0m", if verbose { "ON" } else { "OFF" });
                            }
                        }
                        ":lint" => {
                            if arg.is_empty() {
                                eprintln!("\x1b[1;31mUsage:\x1b[0m :lint <file.omni | file.scroll>");
                            } else {
                                match fs::read_to_string(arg) {
                                    Ok(content) => {
                                        let linter = OmniLinter::new(logger.clone());
                                        let report = linter.lint_source(arg, &content);
                                        if report.is_valid {
                                            println!("\x1b[1;32m✓ 4-Block Structural Check Passed:\x1b[0m {} blocks verified in canonical sequence.", report.blocks_found.len());
                                        } else {
                                            println!("\x1b[1;31m✗ 4-Block Structural Linter Violations:\x1b[0m");
                                            for err in report.errors {
                                                println!("    \x1b[1;31m•\x1b[0m {}", err);
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("\x1b[1;31m✗ File read error:\x1b[0m {}", e),
                                }
                            }
                        }
                        ":load" => {
                            if arg.is_empty() {
                                eprintln!("\x1b[1;31mUsage:\x1b[0m :load <file.omni | file.scroll>");
                            } else {
                                match fs::read_to_string(arg) {
                                    Ok(source) => {
                                        let prev_fps_len = logger.failpoints().len();
                                        let mut lex = Lexer::new(&source, logger.clone());
                                        match lex.tokenize() {
                                            Ok(toks) => {
                                                let mut p = Parser::new(toks, debugger.clone());
                                                match p.parse_program() {
                                                    Ok(prog) => {
                                                        let mut eval = Evaluator::new(env.clone(), p.debugger);
                                                        match eval.eval(&ASTNode::Program(prog)) {
                                                            Ok(res) => {
                                                                env = eval.env;
                                                                debugger = eval.debugger;
                                                                if let RuntimeValue::State(s) = res {
                                                                    current_state = s;
                                                                }
                                                                println!("\x1b[1;32m✓ Loaded and evaluated '{}':\x1b[0m => {}", arg, res.format_colored());
                                                            }
                                                            Err(e) => eprintln!("\x1b[1;31m✗ Evaluation Error in '{}':\x1b[0m {}", arg, e),
                                                        }
                                                    }
                                                    Err(e) => eprintln!("\x1b[1;31m✗ Parser Error in '{}':\x1b[0m {}", arg, e),
                                                }
                                            }
                                            Err(e) => eprintln!("\x1b[1;31m✗ Lexer Error in '{}':\x1b[0m {}", arg, e),
                                        }
                                        let new_fps = logger.failpoints();
                                        if new_fps.len() > prev_fps_len {
                                            for fp in &new_fps[prev_fps_len..] {
                                                let fp_name = fp.failpoint_type.map(|t| t.as_str()).unwrap_or("FAILPOINT");
                                                println!("  \x1b[1;33m[FAILPOINT: {}]\x1b[0m {}", fp_name, fp.message);
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("\x1b[1;31m✗ File read error:\x1b[0m {}", e),
                                }
                            }
                        }
                        other => {
                            eprintln!("\x1b[1;31mUnknown command:\x1b[0m '{}'. Type :help for commands.", other);
                        }
                    }
                    continue;
                }

                // OmniCode Expression / Statement Evaluation Pipeline
                let prev_fps_len = logger.failpoints().len();
                let mut lexer = Lexer::new(input_trimmed, logger.clone());
                match lexer.tokenize() {
                    Ok(tokens) => {
                        let mut parser = Parser::new(tokens, debugger.clone());
                        match parser.parse_program() {
                            Ok(prog) => {
                                let mut evaluator = Evaluator::new(env.clone(), parser.debugger);
                                match evaluator.eval(&ASTNode::Program(prog)) {
                                    Ok(val) => {
                                        env = evaluator.env;
                                        debugger = evaluator.debugger;
                                        if let RuntimeValue::State(s) = val {
                                            current_state = s;
                                        }
                                        println!("=> {}", val.format_colored());
                                    }
                                    Err(err) => {
                                        eprintln!("\x1b[1;31m✗ Runtime Error:\x1b[0m {}", err);
                                    }
                                }
                            }
                            Err(parse_err) => {
                                eprintln!("\x1b[1;31m✗ Syntax Error:\x1b[0m {}", parse_err);
                            }
                        }
                    }
                    Err(lex_err) => {
                        eprintln!("\x1b[1;31m✗ Lexical Error:\x1b[0m {}", lex_err);
                    }
                }

                // Check and print any failpoint alerts triggered during this line
                let all_fps = logger.failpoints();
                if all_fps.len() > prev_fps_len {
                    for fp in &all_fps[prev_fps_len..] {
                        let fp_name = fp.failpoint_type.map(|t| t.as_str()).unwrap_or("FAILPOINT");
                        println!("  \x1b[1;33m[FAILPOINT: {}]\x1b[0m {}", fp_name, fp.message);
                    }
                }

                if verbose {
                    println!("\x1b[0;37m(Active bindings: {}, snapshot depth: {})\x1b[0m", env.bindings().len(), debugger.boundary_stack.len());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\x1b[1;33m^C (Interrupted. Type :exit or press Ctrl+D to quit)\x1b[0m");
                multiline_buffer.clear();
            }
            Err(ReadlineError::Eof) => {
                println!("\x1b[1;36mSealing OmniCode session. Goodbye.\x1b[0m");
                break;
            }
            Err(err) => {
                eprintln!("\x1b[1;31mTerminal Error:\x1b[0m {:?}", err);
                break;
            }
        }
    }

    if let Some(ref path) = history_file {
        let _ = rl.save_history(path);
    }

    Ok(())
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Session Invariants, Teardown Proofs, and Terminal Teardown )
// ==============================================================================
// Terminal REPL engine verified and sealed.
