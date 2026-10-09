// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( CLI Executable Identity, Subsystem Routing & Core Imports )
// ==============================================================================
//! OmniCode CLI Entrypoint
//!
//! Provides the primary terminal interface for the OmniCode language engine:
//!     - Interactive REPL Terminal (default)
//!     - Single-file Script Runner (`run <file.omni>`)
//!     - 4-Block Structural Linter (`lint <file.omni>`)
//!     - Canonical Template Scaffolder (`scaffold <Name>`)
//!     - Ontological Verification Demo (`demo`)

use std::env;
use std::fs;
use std::process;

use omnicode::{
    run_repl, DiagnosticLogger, OmniDebugger, OmniLinter, Restorator,
    AICortex, AILearner, OmniFileType, Lexer, Parser, Evaluator, Environment
};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( CLI Help Display, Version Banner, and Command Route Setup )
// ==============================================================================
fn print_cli_help() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode: Dual Human & Machine Readable Language Engine (v0.1.0)\x1b[0m");
    println!("\x1b[1;36m  Core: Balanced Ternary & Boundary Physics (Rust Engine)\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");
    println!("\x1b[1;33mUSAGE:\x1b[0m");
    println!("  omnicode                    Start the interactive terminal shell (REPL)");
    println!("  omnicode repl               Start the interactive terminal shell (REPL)");
    println!("  omnicode run <file>         Execute an OmniCode source (.omni) or scroll (.scroll)");
    println!("  omnicode lint <file>        Validate 4-Block architecture compliance (.omni / .scroll)");
    println!("  omnicode restore <file>     Auto-heal syntax, boundary, and block anomalies");
    println!("  omnicode ai \"<prompt>\"      Transduce prompt with AI Cortex, self-heal, execute & inscribe logs");
    println!("  omnicode scrolls            Inspect episodic memory bank and learning statistics");
    println!("  omnicode scaffold <Name> [--type omni|scroll] Generate canonical 4-Block template");
    println!("  omnicode filetypes          Display all formal OmniCode file types and MIME definitions");
    println!("  omnicode demo               Run the built-in ontological verification suite");
    println!("  omnicode --help, -h         Show this help information");
    println!("  omnicode --version, -v      Display engine version\n");
    println!("\x1b[1;33mREPL FEATURES:\x1b[0m");
    println!("  • Real-time evaluation of balanced ternary expressions (-1, 0, +1, NULL)");
    println!("  • Boundary delimiter matching with multiline input buffering");
    println!("  • Interactive failpoint alerts and transaction snapshot rollback");
    println!("  • Live symbol inspection via `:env` and AST inspection via `:ast <expr>`");
    println!("  • Persistent command line history\n");
}

fn print_version() {
    println!("omnicode 0.1.0 (Rust Core: Balanced Ternary & Boundary Physics)");
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Command Handlers: Script Runner, Linter, Scaffolder, Ontological Demo )
// ==============================================================================
fn run_file(path: &str) {
    let ft = OmniFileType::from_path(path);
    let title = ft.map(|f| f.display_name()).unwrap_or("OmniCode source file");
    println!("\x1b[1;36mExecuting {}:\x1b[0m {}", title, path);
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[1;31m✗ File read error:\x1b[0m Could not open '{}': {}", path, e);
            process::exit(1);
        }
    };

    let logger = DiagnosticLogger::new(false);

    // 1. Structural 4-Block Lint Check
    let linter = OmniLinter::new(logger.clone());
    let lint_report = linter.lint_source(path, &content);
    if !lint_report.is_valid {
        eprintln!("\x1b[1;33m⚠ Warning: File failed 4-Block architectural validation:\x1b[0m");
        for err in &lint_report.errors {
            eprintln!("    • {}", err);
        }
    } else {
        println!("\x1b[1;32m✓ 4-Block Structural Check:\x1b[0m Valid ({} blocks verified).", lint_report.blocks_found.len());
    }

    // 2. Tokenization
    let mut lexer = Lexer::new(&content, logger.clone());
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Lexical Error:\x1b[0m {}", e);
            process::exit(1);
        }
    };

    // 3. Parsing
    let debugger = OmniDebugger::new(logger.clone(), 10);
    let mut parser = Parser::new(tokens, debugger);
    let prog = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Parser Error:\x1b[0m {}", e);
            process::exit(1);
        }
    };

    // 4. Execution
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    match evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)) {
        Ok(result) => {
            println!("\x1b[1;32m✓ Execution Succeeded:\x1b[0m => {}", result.format_colored());
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Evaluation Error:\x1b[0m {}", e);
            process::exit(1);
        }
    }

    // 5. Failpoints report
    if logger.has_failpoints() {
        println!("\n\x1b[1;33m--- Trapped Diagnostic Failpoints ({}) ---\x1b[0m", logger.failpoints().len());
        for fp in logger.failpoints() {
            let fp_name = fp.failpoint_type.map(|t| t.as_str()).unwrap_or("FAILPOINT");
            println!("  * [{} µs] \x1b[1;31m[{}]\x1b[0m [{}] {}", fp.elapsed_micros, fp.channel, fp_name, fp.message);
        }
    }
}

fn run_lint_file(path: &str) {
    let ft = OmniFileType::from_path(path);
    let title = ft.map(|f| f.display_name()).unwrap_or("OmniCode file");
    println!("\x1b[1;33m--- Linting 4-Block Structural Architecture ({title}): {} ---\x1b[0m", path);
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[1;31m✗ File read error:\x1b[0m Could not open '{}': {}", path, e);
            process::exit(1);
        }
    };

    let logger = DiagnosticLogger::new(false);
    let linter = OmniLinter::new(logger);
    let report = linter.lint_source(path, &content);

    if report.is_valid {
        println!("\x1b[1;32m✓ 4-Block Validation PASSED\x1b[0m");
        println!("  Blocks verified in sequence:");
        for b in &report.blocks_found {
            println!("    - Line {}: {}", b.line_number, b.kind.name());
        }
    } else {
        println!("\x1b[1;31m✗ 4-Block Validation FAILED\x1b[0m");
        for err in &report.errors {
            eprintln!("    • {}", err);
        }
        process::exit(1);
    }
}

fn run_scaffold(name: &str, file_type: Option<&str>) {
    let ft = file_type
        .and_then(OmniFileType::from_extension)
        .unwrap_or(OmniFileType::SourceReel);
    let scaffolded = match ft {
        OmniFileType::SourceReel => OmniLinter::auto_scaffold_omni(name, "User-defined module"),
        OmniFileType::EpisodicScroll => OmniLinter::auto_scaffold_scroll(name, "User cognitive discourse intent"),
    };
    println!("{}", scaffolded);
}

fn run_filetypes() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode File Type System & Taxonomy Manifest                                \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    for (i, ft) in OmniFileType::all().iter().enumerate() {
        println!("  \x1b[1;33m[{}]\x1b[0m \x1b[1;32m.{}\x1b[0m  —  \x1b[1m{}\x1b[0m", i + 1, ft.extension(), ft.display_name());
        println!("      MIME:        \x1b[36m{}\x1b[0m", ft.mime_type());
        println!("      Language ID: \x1b[35m{}\x1b[0m", ft.language_id());
        println!("      Purpose:     {}\n", ft.description());
    }
}

fn run_demo() {
    println!("\x1b[1;36m=====================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode: Ontological Conceptual Testbed           \x1b[0m");
    println!("\x1b[1;36m=====================================================\x1b[0m\n");

    let demo_code = fs::read_to_string("examples/canonical_notation.omni")
        .unwrap_or_else(|_| OmniLinter::auto_scaffold_omni("DemoModule", "Ontological verification module"));


    println!("\x1b[1;33m--- Running 4-Block Structural Linter ---\x1b[0m");
    let logger = DiagnosticLogger::new(true);
    let linter = OmniLinter::new(logger.clone());
    let lint_report = linter.lint_source("demo_code", &demo_code);

    if lint_report.is_valid {
        println!("\x1b[1;32m✓ 4-Block Structural Check Passed:\x1b[0m Found {} blocks in order.", lint_report.blocks_found.len());
    } else {
        println!("\x1b[1;31m✗ Structural Linter Errors:\x1b[0m {:?}", lint_report.errors);
    }

    println!("\n\x1b[1;33m--- Running Code Evaluation Pipeline ---\x1b[0m");
    let mut lexer = Lexer::new(&demo_code, logger.clone());

    match lexer.tokenize() {
        Ok(tokens) => {
            println!("\x1b[1;32m✓ Lexer Tokenization Succeeded:\x1b[0m ({} tokens)", tokens.len());
            let debugger = OmniDebugger::new(logger.clone(), 5);
            let mut parser = Parser::new(tokens, debugger);

            match parser.parse_program() {
                Ok(prog) => {
                    println!("\x1b[1;32m✓ Parser Succeeded:\x1b[0m Generated AST with {} statements", prog.statements.len());
                    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
                    match evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)) {
                        Ok(res) => {
                            println!("\n\x1b[1;32m✓ Execution Final Result:\x1b[0m => {}", res.format_colored());
                        }
                        Err(e) => eprintln!("\x1b[1;31m✗ Evaluator Error:\x1b[0m {}", e),
                    }
                }
                Err(e) => eprintln!("\x1b[1;31m✗ Parser Error:\x1b[0m {}", e),
            }
        }
        Err(e) => eprintln!("\x1b[1;31m✗ Lexer Error:\x1b[0m {}", e),
    }

    if logger.has_failpoints() {
        println!("\n\x1b[1;33m--- Diagnostic Failpoints Caught During Run ---\x1b[0m");
        for fp in logger.failpoints() {
            println!("  * [{}] {}", fp.channel, fp.message);
        }
    } else {
        println!("\n\x1b[1;32m✓ No unexpected failpoints encountered during demo run.\x1b[0m");
    }
}

fn run_restore(path: &str, in_place: bool) {
    println!("\x1b[1;36mAutonomous Self-Healing Restoration:\x1b[0m {}", path);
    let logger = DiagnosticLogger::new(false);
    let restorator = Restorator::new(logger);

    match restorator.restore_file(path, in_place) {
        Ok(report) => {
            if !report.was_modified {
                println!("\x1b[1;32m✓ Source is already healthy:\x1b[0m No repairs needed.");
            } else {
                println!("\x1b[1;33m⚠ Repairs Applied ({}):\x1b[0m", report.repairs_applied.len());
                for repair in &report.repairs_applied {
                    println!("    • L{}: [{:?}] {}", repair.line, repair.kind, repair.description);
                }
                if in_place {
                    println!("\x1b[1;32m✓ Healed file written in-place to {}\x1b[0m", path);
                } else {
                    println!("\n\x1b[1;36m--- Restored Content Preview ---\x1b[0m\n{}", report.restored_content);
                }
            }

            if report.verified {
                println!("\x1b[1;32m✓ Post-Restoration Verification PASSED:\x1b[0m Code executes to => +1:PROCEED");
            } else if !report.errors.is_empty() {
                println!("\x1b[1;33m⚠ Verification Diagnostics:\x1b[0m {:?}", report.errors);
            }
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Restoration Failed:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}

fn run_ai(prompt: &str) {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: Cognitive Cortex (Logger + Debugger + Restorator + Transducer)\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");
    println!("\x1b[1;33m[Human Layer Discourse Prompt]:\x1b[0m\n  \"{}\"\n", prompt);

    let mut cortex = AICortex::new();

    match cortex.process_prompt(prompt) {
        Ok(res) => {
            if !res.repairs_applied.is_empty() {
                println!("\x1b[1;33m⚠ Restorator Immune Repairs Applied ({}):\x1b[0m", res.repairs_applied.len());
                for r in &res.repairs_applied {
                    println!("    • L{}: [{:?}] {}", r.line, r.kind, r.description);
                }
                println!();
            }

            println!("\x1b[1;32m✓ Synthesized 4-Block OmniCode Turing Reel:\x1b[0m");
            println!("-------------------------------------------------------------------------------");
            println!("{}", res.synthesized_omnicode.trim());
            println!("-------------------------------------------------------------------------------");
            println!("\x1b[1;32m✓ Machine Quad-State Status:\x1b[0m => +1:{}", res.execution_state.macro_op());
            println!("\n\x1b[1;35m[Machine Layer Response]:\x1b[0m");
            println!("  \"{}\"\n", res.natural_response);
            println!("\x1b[1;34m[Episodic Learning Inscription]:\x1b[0m");
            println!("  • OmniCode Reel (.scroll): {:?}", res.scroll_path);
            println!("  • Traditional Telemetry (.log): {:?}", res.log_path);
            println!("  (Session ID: {})\n", res.session_id);
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ AI Cortex Error:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}

fn run_scrolls() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: Episodic Memory Bank & Experiential Learning Archive           \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let learner = AILearner::new("scrolls");
    match learner.digest_experiences() {
        Ok(summary) => {
            println!("\x1b[1;33mEPISODIC LEARNING SUMMARY:\x1b[0m");
            println!("  • Total Inscribed Episodes:     {}", summary.total_episodes);
            println!("  • State Distribution:           Proceed (+1): {} | Halt (NULL): {} | Rollback (-1): {} | Await (0): {}",
                summary.proceed_count, summary.halt_count, summary.rollback_count, summary.await_count
            );
            println!("  • Restorator Repairs Learned:   {}\n", summary.total_repairs_learned);

            if summary.experiences.is_empty() {
                println!("  (No scrolls recorded yet in `scrolls/`. Run `omnicode ai \"...\"` to generate experiences.)\n");
            } else {
                println!("\x1b[1;33mRECENT EPISODES:\x1b[0m");
                for (i, exp) in summary.experiences.iter().enumerate() {
                    let log_status = if exp.log_path.is_some() { "yes" } else { "no" };
                    println!(
                        "  [{:>2}] Session: {:<12} | State: +1:{:<8} | Repairs: {:>2} | Failpoints: {:>2} | .log: {}",
                        i + 1, exp.session_id, exp.final_state.macro_op(), exp.repair_count, exp.failpoint_count, log_status
                    );
                    println!("       Prompt: \"{}\"", exp.prompt);
                    println!("       Scroll: {:?}\n", exp.scroll_path);
                }
            }
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Failed to read scrolls:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( CLI Main Entrypoint Dispatcher & Process Termination )
// ==============================================================================
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        // Default behavior: launch interactive terminal environment (REPL)
        if let Err(e) = run_repl() {
            eprintln!("\x1b[1;31mFatal REPL Error:\x1b[0m {}", e);
            process::exit(1);
        }
        return;
    }

    match args[1].as_str() {
        "repl" => {
            if let Err(e) = run_repl() {
                eprintln!("\x1b[1;31mFatal REPL Error:\x1b[0m {}", e);
                process::exit(1);
            }
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode run <file.omni>");
                process::exit(1);
            }
            run_file(&args[2]);
        }
        "lint" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode lint <file.omni>");
                process::exit(1);
            }
            run_lint_file(&args[2]);
        }
        "restore" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode restore <file.omni> [--write]");
                process::exit(1);
            }
            let in_place = args.iter().any(|a| a == "--write" || a == "-w");
            run_restore(&args[2], in_place);
        }
        "ai" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing prompt. Usage: omnicode ai \"<natural language sentence>\"");
                process::exit(1);
            }
            let prompt = args[2..].join(" ");
            run_ai(&prompt);
        }
        "scrolls" | "history" | "learn" => {
            run_scrolls();
        }
        "filetypes" | "types" => {
            run_filetypes();
        }
        "scaffold" => {
            let (name, kind) = if args.len() >= 4 && (args[2] == "--type" || args[2] == "-t") {
                (args.get(4).map(|s| s.as_str()).unwrap_or("MyModule"), Some(args[3].as_str()))
            } else if args.len() >= 4 && (args[3] == "--type" || args[3] == "-t") {
                (args[2].as_str(), args.get(4).map(|s| s.as_str()))
            } else if args.len() >= 4 && (args[2] == "scroll" || args[2] == "omni") {
                (args[3].as_str(), Some(args[2].as_str()))
            } else if args.len() >= 3 {
                (args[2].as_str(), None)
            } else {
                ("MyModule", None)
            };
            run_scaffold(name, kind);
        }
        "demo" => {
            run_demo();
        }
        "--help" | "-h" | "help" => {
            print_cli_help();
        }
        "--version" | "-v" | "version" => {
            print_version();
        }
        other => {
            eprintln!("\x1b[1;31mUnknown argument:\x1b[0m '{}'", other);
            print_cli_help();
            process::exit(1);
        }
    }
}
