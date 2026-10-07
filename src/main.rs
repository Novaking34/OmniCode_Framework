//! OmniCode CLI & Interactive Conceptual Testbed
//!
//! Demonstrates the complete foundational syntax and semantics:
//!     - Balanced Ternary (-1 Rollback, 0 Await, +1 Proceed, NULL Halt)
//!     - 6 Morphological Boundary Types
//!     - First-Class Asides (Machine intent)
//!     - 3D Coordinate Operators (=, !=, ~=, -->, <--, ^--, v--, !, ::, :, |, --)

use omnicode::{DiagnosticLogger, OmniDebugger, Lexer, Parser, Evaluator, Environment};

fn main() {
    println!("\x1b[1;36m=====================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode: AI-Native Ontological Language Engine    \x1b[0m");
    println!("\x1b[1;36m  Core: Balanced Ternary & Boundary Physics (Rust)   \x1b[0m");
    println!("\x1b[1;36m=====================================================\x1b[0m\n");

    let demo_code = r#"
# 1. Immutable Invariant Definition (Axiom)
PI < 3.14159 > ( immutable mathematical constant )

# 2. Mutable Entity Definition with First-Class Aside and Context
Buffer [ 4096 ] ( zero-copy ring buffer ) { aligned.64 }

# 3. Being Relation (Identity)
status [ +1 = +1 ] ( verify identity holds )

# 4. Adversative Caveat (! BUT) with Forward Causation (-->)
Transaction [ +1 ] ! [ +1 ] --> Output [ +1 ]
"#;

    println!("\x1b[1;33m--- Running Conceptual Code Sample ---\x1b[0m");
    println!("Source Code:\n{}\n", demo_code.trim());

    let logger = DiagnosticLogger::new(true);
    let mut lexer = Lexer::new(demo_code, logger.clone());

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
                            println!("\n\x1b[1;32m✓ Execution Final Result:\x1b[0m {:?}", res);
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
