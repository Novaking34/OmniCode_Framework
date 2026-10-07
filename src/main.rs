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
    run_repl, DiagnosticLogger, OmniDebugger, OmniLinter, Lexer, Parser, Evaluator, Environment
};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( CLI Help Display, Version Banner, and Command Route Setup )
// ==============================================================================
fn print_cli_help() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode: AI-Native Ontological Language Engine (v0.1.0)\x1b[0m");
    println!("\x1b[1;36m  Core: Balanced Ternary & Boundary Physics (Rust Engine)\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");
    println!("\x1b[1;33mUSAGE:\x1b[0m");
    println!("  omnicode                    Start the interactive terminal shell (REPL)");
    println!("  omnicode repl               Start the interactive terminal shell (REPL)");
    println!("  omnicode run <file.omni>    Execute an OmniCode source file");
    println!("  omnicode lint <file.omni>   Validate 4-Block architecture compliance");
    println!("  omnicode scaffold <Name>    Generate a canonical 4-Block source file template");
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
    println!("\x1b[1;36mExecuting OmniCode source file:\x1b[0m {}", path);
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
    println!("\x1b[1;33m--- Linting 4-Block Structural Architecture: {} ---\x1b[0m", path);
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
            println!("    • {}", err);
        }
        process::exit(1);
    }
}

fn run_scaffold(name: &str) {
    let scaffolded = OmniLinter::auto_scaffold_omni(name, "User-defined module");
    println!("{}", scaffolded);
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
        "scaffold" => {
            let name = if args.len() >= 3 { &args[2] } else { "MyModule" };
            run_scaffold(name);
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
