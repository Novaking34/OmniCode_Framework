// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Evaluator Core Dispatcher, AST Execution Loop & Module Loader )
// ==============================================================================
//! OmniCode Runtime: Evaluator Core Engine
//!
//! # Architectural Scope & Purpose:
//! Serves as the central execution orchestrator of the OmniCode VM. Coordinates
//! AST node dispatch across the specialized sub-engines:
//! - SVO Verbs & Memory Vessels (`verbs.rs`)
//! - Relational Algebra & Bounds (`relations.rs`)
//! - Directional Causal Propagation (`vectors.rs`)
//! - Logical Connectives & Adversative Caveats (`connectives.rs`)
//! - Universal Decoders & Trit Converters (`universal/decoder.rs`)
//!
//! Also manages dynamic module resolution and topological imports across `.omni`
//! reels and `.tape` ribbons.
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
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Modular Architecture**: Moving sub-engines into dedicated modules eliminates
//!    monolithic complexity while guaranteeing complete unit test coverage.
// ==============================================================================

use std::collections::HashSet;
use std::path::PathBuf;

use crate::ast::nodes::*;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::universal::extract_symbol_name;
use crate::runtime::evaluator::relations::eval_relation;
use crate::runtime::evaluator::vectors::eval_directional;
use crate::runtime::evaluator::connectives::eval_connective;
use crate::runtime::evaluator::verbs::eval_svo_verb_or_vessel;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Evaluator Struct Definition & Initialization )
// ==============================================================================

/// Central execution engine coordinating AST evaluation, transactions, and modules.
pub struct Evaluator {
    pub env: Environment,
    pub debugger: OmniDebugger,
    pub loaded_modules: HashSet<String>,
}

impl Evaluator {
    /// Creates a new Evaluator with the given environment and debugger.
    pub fn new(env: Environment, debugger: OmniDebugger) -> Self {
        Self {
            env,
            debugger,
            loaded_modules: HashSet::new(),
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Main Evaluation Loop, Module Loader & Helper Dispatchers )
// ==============================================================================

impl Evaluator {
    /// Evaluates an AST node and yields a `RuntimeValue` or runtime error.
    pub fn eval(&mut self, node: &ASTNode) -> Result<RuntimeValue, String> {
        match node {
            // ------------------------------------------------------------------
            // Literals & Primitives
            // ------------------------------------------------------------------
            ASTNode::Ternary(state) => {
                self.debugger.inspect_state(*state, "TernaryLiteral");
                Ok(RuntimeValue::State(*state))
            }

            ASTNode::Number(val) => Ok(RuntimeValue::Number(*val)),

            ASTNode::StringLit(s) => Ok(RuntimeValue::Text(s.clone())),

            ASTNode::Identifier(name) => {
                if let Some(val) = self.env.get(name) {
                    Ok(val.clone())
                } else if name.eq_ignore_ascii_case("proceed") || name.eq_ignore_ascii_case("yes") {
                    Ok(RuntimeValue::State(QuadState::Proceed))
                } else if name.eq_ignore_ascii_case("rollback") || name.eq_ignore_ascii_case("no") {
                    Ok(RuntimeValue::State(QuadState::Rollback))
                } else if name.eq_ignore_ascii_case("await") {
                    Ok(RuntimeValue::State(QuadState::Await))
                } else if name.eq_ignore_ascii_case("halt") {
                    Ok(RuntimeValue::State(QuadState::Halt))
                } else {
                    self.debugger.logger.log("IDENTIFIER", format!("Symbolic ontological reference: '{name}'"));
                    Ok(RuntimeValue::Text(name.clone()))
                }
            }

            ASTNode::Polarity(p) => {
                let formatted = match p.position {
                    crate::core::operators::AffixPosition::Prefix => format!("{}{}", p.polarity.symbol(), p.name),
                    crate::core::operators::AffixPosition::Postfix => format!("{}{}", p.name, p.polarity.symbol()),
                };
                self.debugger.logger.log("POLARITY", format!("Evaluated polarity entity: {formatted}"));
                if p.name.eq_ignore_ascii_case("proceed") {
                    Ok(RuntimeValue::State(QuadState::Proceed))
                } else if p.name.eq_ignore_ascii_case("rollback") {
                    Ok(RuntimeValue::State(QuadState::Rollback))
                } else if p.name.eq_ignore_ascii_case("await") {
                    Ok(RuntimeValue::State(QuadState::Await))
                } else if p.name.eq_ignore_ascii_case("halt") {
                    Ok(RuntimeValue::State(QuadState::Halt))
                } else if let Some(val) = self.env.get(&formatted) {
                    Ok(val.clone())
                } else if let Some(val) = self.env.get(&p.name) {
                    Ok(val.clone())
                } else {
                    Ok(RuntimeValue::Text(formatted))
                }
            }

            ASTNode::BoundaryCast(bc) => {
                let (open, close) = bc.boundary.delimiters();
                let cast_str = format!("{}{}{}", open, bc.polarity.symbol(), close);
                self.debugger.logger.log("CAST", format!("Evaluated boundary cast: {cast_str}"));
                Ok(RuntimeValue::Text(cast_str))
            }

            ASTNode::Aside(aside) => {
                self.debugger.logger.log("ASIDE", format!("Evaluating Aside context: '{}'", aside.intent));
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            // ------------------------------------------------------------------
            // Boundary Scopes & Transaction Wrapping
            // ------------------------------------------------------------------
            ASTNode::Boundary(b) => {
                // Check if this boundary is a pure symbol reference (like [ ROLL 1 ], [ Verify ])
                if let Some(sym_name) = extract_symbol_name(node) {
                    if let Some(val) = self.env.get(&sym_name) {
                        return Ok(val.clone());
                    }
                }

                self.env.begin_transaction();
                let mut last_val = RuntimeValue::State(QuadState::Proceed);

                for el in &b.elements {
                    last_val = self.eval(el)?;
                    if let RuntimeValue::State(s) = last_val {
                        if s == QuadState::Rollback {
                            self.env.rollback_transaction()?;
                            self.debugger.logger.failpoint(
                                FailpointType::RollbackTriggered,
                                "EVALUATOR",
                                format!("Boundary {} unwound due to -1 (Rollback)", b.boundary_type),
                            );
                            return Ok(RuntimeValue::State(QuadState::Rollback));
                        } else if s == QuadState::Halt {
                            self.debugger.logger.failpoint(
                                FailpointType::NullTrap,
                                "EVALUATOR",
                                format!("Boundary {} trapped by NULL / Void", b.boundary_type),
                            );
                            return Ok(RuntimeValue::State(QuadState::Halt));
                        }
                    }
                }

                self.env.commit_transaction();
                Ok(last_val)
            }

            // ------------------------------------------------------------------
            // Entity Definitions & SVO Action Verbs
            // ------------------------------------------------------------------
            ASTNode::Entity(entity) => {
                // Try dispatching as an SVO Action Verb or Memory Vessel
                if let Some(verb_res) = eval_svo_verb_or_vessel(self, entity)? {
                    return Ok(verb_res);
                }

                let is_imm = entity.definition.boundary_type.is_immutable();
                if is_imm {
                    self.debugger.register_immutable(&entity.name);
                } else {
                    self.debugger.assert_mutation_permitted(&entity.name)?;
                }

                if let Some(ref aside) = entity.aside {
                    self.debugger.logger.log(
                        "ASIDE",
                        format!("Entity '{}' attached intent: ({})", entity.name, aside.intent),
                    );
                }

                let val = self.eval(&ASTNode::Boundary(entity.definition.clone()))?;
                self.env.set(&entity.name, val.clone());
                self.debugger.logger.log("ENTITY", format!("Defined entity '{}' = {:?}", entity.name, val));
                Ok(val)
            }

            // ------------------------------------------------------------------
            // Relational Algebra
            // ------------------------------------------------------------------
            ASTNode::Relation(rel) => eval_relation(self, rel),

            // ------------------------------------------------------------------
            // Logical Connectives & Adversative Caveats
            // ------------------------------------------------------------------
            ASTNode::Connective(conn) => eval_connective(self, conn),

            // ------------------------------------------------------------------
            // Directional Causal Propagation
            // ------------------------------------------------------------------
            ASTNode::Directional(dir) => eval_directional(self, dir),

            // ------------------------------------------------------------------
            // Scalar Arithmetic
            // ------------------------------------------------------------------
            ASTNode::Arithmetic(arith) => {
                let left_val = self.eval(&arith.left)?;
                let right_val = self.eval(&arith.right)?;

                match (&left_val, &right_val) {
                    (RuntimeValue::Number(l), RuntimeValue::Number(r)) => {
                        let res = match arith.op {
                            ArithmeticOp::Add => l + r,
                            ArithmeticOp::Sub => l - r,
                            ArithmeticOp::Mul => l * r,
                            ArithmeticOp::Div => {
                                if *r == 0.0 {
                                    self.debugger.logger.failpoint(
                                        FailpointType::NullTrap,
                                        "EVALUATOR",
                                        "Division by zero traps as NULL/Void",
                                    );
                                    return Ok(RuntimeValue::State(QuadState::Halt));
                                }
                                l / r
                            }
                        };
                        Ok(RuntimeValue::Number(res))
                    }
                    _ => {
                        self.debugger.logger.log(
                            "ARITHMETIC",
                            format!("Symbolic arithmetic between {:?} and {:?}", left_val, right_val),
                        );
                        Ok(right_val)
                    }
                }
            }

            // ------------------------------------------------------------------
            // Unary Inversion
            // ------------------------------------------------------------------
            ASTNode::UnaryNot(un) => {
                let s = self.eval_to_state(&un.operand)?;
                Ok(RuntimeValue::State(s.invert()))
            }

            // ------------------------------------------------------------------
            // Program Block Execution
            // ------------------------------------------------------------------
            ASTNode::Program(prog) => {
                let mut last = RuntimeValue::State(QuadState::Proceed);
                for stmt in &prog.statements {
                    last = self.eval(stmt)?;
                }
                Ok(last)
            }

            // ------------------------------------------------------------------
            // Module Declaration
            // ------------------------------------------------------------------
            ASTNode::Module(module) => {
                self.debugger.logger.log("MODULE", format!("Mounting Module: '{}'", module.name));
                self.env.set("Module", RuntimeValue::Text(module.name.clone()));
                if let Some(ref aside) = module.aside {
                    self.debugger.logger.log("ASIDE", format!("Module intent: ({})", aside.intent));
                }
                for field in &module.fields {
                    self.eval(field)?;
                }
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            // ------------------------------------------------------------------
            // Process Tape Declaration
            // ------------------------------------------------------------------
            ASTNode::Tape(tape) => {
                self.debugger.logger.log("TAPE", format!("Mounting Process Tape: '{}'", tape.name));
                self.env.set("Tape", RuntimeValue::Text(tape.name.clone()));
                if let Some(ref aside) = tape.aside {
                    self.debugger.logger.log("ASIDE", format!("Tape intent: ({})", aside.intent));
                }
                for clause in &tape.clauses {
                    self.eval(clause)?;
                }
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            // ------------------------------------------------------------------
            // 4-Block Section Execution
            // ------------------------------------------------------------------
            ASTNode::Section(section) => {
                self.debugger.logger.log("SECTION", format!("Entering 4-Block Section: {}", section.kind.name()));
                if let Some(ref aside) = section.aside {
                    self.debugger.logger.log("ASIDE", format!("Section intent: ({})", aside.intent));
                }
                let mut last = RuntimeValue::State(QuadState::Proceed);
                for element in &section.elements {
                    last = self.eval(element)?;
                }
                Ok(last)
            }

            // ------------------------------------------------------------------
            // Discrete Operational Roll Execution
            // ------------------------------------------------------------------
            ASTNode::Roll(roll) => {
                self.debugger.logger.log("ROLL", format!("Executing ROLL {}: {}", roll.id, roll.label));
                if let Some(ref aside) = roll.aside {
                    self.debugger.logger.log("ASIDE", format!("Roll intent: ({})", aside.intent));
                }
                let val = self.eval(&roll.action)?;
                self.env.set(&format!("ROLL {}", roll.id), val.clone());
                Ok(val)
            }

            // ------------------------------------------------------------------
            // Closing Fourfold Commit Status Gate
            // ------------------------------------------------------------------
            ASTNode::CommitStatus(cs) => {
                self.debugger.logger.log("COMMIT", "Evaluating Fourfold Commit Status Gate");
                if let Some(ref aside) = cs.aside {
                    self.debugger.logger.log("ASIDE", format!("Commit intent: ({})", aside.intent));
                }
                let mut status = QuadState::Proceed;
                for case in &cs.cases {
                    let val = self.eval(case)?;
                    let state = val.to_quad_state();
                    if state == QuadState::Proceed {
                        status = QuadState::Proceed;
                        break;
                    } else if state == QuadState::Rollback {
                        status = QuadState::Rollback;
                    }
                }
                Ok(RuntimeValue::State(status))
            }

            // ------------------------------------------------------------------
            // Cardinal Interrogatives (WHO, WHAT, WHEN, WHERE, WHY, HOW)
            // ------------------------------------------------------------------
            ASTNode::Cardinal(cardinal) => {
                let kw = cardinal.kind.keyword();
                let val = self.eval(&cardinal.payload)?;
                self.debugger.logger.log("CARDINAL", format!("{kw} clause registered: {:?}", val));
                self.env.set(kw, val.clone());
                Ok(val)
            }

            // ------------------------------------------------------------------
            // Operational SVO Verbs (INSCRIBE, TRANSDUCE, GUARD, PROJECT, SEQUENCE)
            // ------------------------------------------------------------------
            ASTNode::Verb(v) => {
                self.debugger.logger.log("VERB", format!("Executing SVO Action Verb: {}", v.verb.name()));
                let obj_val = self.eval(&v.object)?;
                if let Some(ref target) = v.target {
                    if let Some(target_name) = extract_symbol_name(target) {
                        self.env.set(&target_name, obj_val.clone());
                    }
                }
                Ok(obj_val)
            }

            // ------------------------------------------------------------------
            // Adversative Guard Assertions (! GUARD)
            // ------------------------------------------------------------------
            ASTNode::Guard(g) => {
                let cond_val = self.eval(&g.condition)?;
                let cond_state = cond_val.to_quad_state();
                if cond_state == QuadState::Proceed {
                    self.eval(&g.on_proceed)
                } else if let Some(ref fallback) = g.on_fallback {
                    self.eval(fallback)
                } else {
                    Ok(RuntimeValue::State(QuadState::Rollback))
                }
            }

            // ------------------------------------------------------------------
            // MMIO Hardware Bus Locators (::@)
            // ------------------------------------------------------------------
            ASTNode::Hardware(hw) => {
                let target_val = self.eval(&hw.target)?;
                self.debugger.logger.log("HARDWARE", format!("MMIO Address 0x{:X} mapped to {:?}", hw.address, target_val));
                Ok(RuntimeValue::Number(hw.address as f64))
            }

            // ------------------------------------------------------------------
            // Invariant Ontological Type Definitions
            // ------------------------------------------------------------------
            ASTNode::InvariantType(inv) => {
                self.debugger.logger.log("INVARIANT", format!("Registered Invariant Type: <{}>", inv.name));
                self.env.set(&inv.name, RuntimeValue::Text(format!("<{}>", inv.name)));
                if let Some(ref aside) = inv.aside {
                    self.debugger.logger.log("ASIDE", format!("Type intent: ({})", aside.intent));
                }
                for invariant in &inv.invariants {
                    self.eval(invariant)?;
                }
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            // ------------------------------------------------------------------
            // Ontological Analogy Proportions (A : B :: C : D)
            // ------------------------------------------------------------------
            ASTNode::Analogy(_) => {
                self.debugger.logger.log("ANALOGY", "Evaluating ontological proportion");
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            // ------------------------------------------------------------------
            // Dual Substrate Equivalence (Bits <=> Trits)
            // ------------------------------------------------------------------
            ASTNode::Substrate(sub) => {
                self.debugger.logger.log(
                    "SUBSTRATE",
                    format!("Dual substrate: {} physical bits <=> {} virtual trits", sub.physical_bits, sub.virtual_trits),
                );
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            // ------------------------------------------------------------------
            // Canonical Exemplar Mappings (Scalar <=> Trits)
            // ------------------------------------------------------------------
            ASTNode::Exemplar(ex) => {
                self.debugger.logger.log(
                    "EXEMPLAR",
                    format!("Canonical exemplar: {} <=> {:?}", ex.scalar, ex.trits),
                );
                Ok(RuntimeValue::State(QuadState::Proceed))
            }
        }
    }

    /// Helper evaluating a node and coercing its result to a `QuadState`.
    pub fn eval_to_state(&mut self, node: &ASTNode) -> Result<QuadState, String> {
        let val = self.eval(node)?;
        Ok(val.to_quad_state())
    }

    /// Recursively extracts module identifier names from an AST node.
    pub fn extract_module_names(&self, node: &ASTNode, names: &mut Vec<String>) {
        match node {
            ASTNode::Identifier(name) => {
                if !name.is_empty() {
                    names.push(name.clone());
                }
            }
            ASTNode::Boundary(b) => {
                for el in &b.elements {
                    self.extract_module_names(el, names);
                }
            }
            ASTNode::Connective(conn) => {
                self.extract_module_names(&conn.left, names);
                self.extract_module_names(&conn.right, names);
            }
            _ => {}
        }
    }

    /// Resolves an omnicode module name into a physical file path.
    pub fn resolve_module_path(&self, module_name: &str) -> Option<PathBuf> {
        let parts: Vec<&str> = module_name.split('.').collect();
        let rel_slash = parts.join("/").to_lowercase();
        let rel_dot = module_name.to_lowercase();

        let candidates = [
            format!("lib/{rel_slash}.omni"),
            format!("lib/{rel_dot}.omni"),
            format!("lexicon/{rel_slash}.omni"),
            format!("lexicon/{rel_dot}.omni"),
            format!("{rel_slash}.omni"),
            format!("{rel_dot}.omni"),
            format!("examples/{rel_slash}.omni"),
            format!("examples/{rel_dot}.omni"),
        ];

        for cand in &candidates {
            let path = crate::core::filetype::resolve_track_path(cand);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    /// Imports and executes an omnicode module into the active environment.
    pub fn import_module(&mut self, module_name: &str) -> Result<RuntimeValue, String> {
        if self.loaded_modules.contains(module_name) {
            self.debugger.logger.log(
                "IMPORT",
                format!("Module '{module_name}' already loaded; skipping redundant load"),
            );
            return Ok(RuntimeValue::State(QuadState::Proceed));
        }

        let path = match self.resolve_module_path(module_name) {
            Some(p) => p,
            None => {
                let msg = format!("Module '{module_name}' could not be resolved in lib/ or working directory");
                self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "IMPORT", &msg);
                return Err(format!("FAILPOINT: {msg}"));
            }
        };

        let content = std::fs::read_to_string(&path).map_err(|e| {
            let msg = format!("Failed to read module '{module_name}' at {:?}: {e}", path);
            self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "IMPORT", &msg);
            format!("FAILPOINT: {msg}")
        })?;

        self.debugger.logger.log(
            "IMPORT",
            format!("Loading module '{module_name}' from {:?}", path),
        );

        // Track module load immediately to protect against circular imports
        self.loaded_modules.insert(module_name.to_string());

        let mut lexer = Lexer::new(&content, self.debugger.logger.clone());
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens, self.debugger.clone());
        let prog = parser.parse_program()?;

        let result = self.eval(&ASTNode::Program(prog))?;

        self.debugger.logger.log(
            "IMPORT",
            format!("Successfully loaded module '{module_name}' (=> {:?})", result),
        );

        Ok(RuntimeValue::State(QuadState::Proceed))
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Execution Invariant Proofs, Scripture Witness & In-Module Verification Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    fn create_test_evaluator() -> Evaluator {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let env = Environment::new();
        Evaluator::new(env, debugger)
    }

    /// Witness Proof 1: Exodus 31:18 KJV
    /// Bijective Radix-3 Transduction (<=>) Canonical Exemplar Letter 'A' (65).
    #[test]
    fn test_bijective_transduction_exemplar() {
        let mut eval = create_test_evaluator();
        let src = "65 <=> [ -1, +1, +1, -1, +1 ]";
        let mut lexer = Lexer::new(src, eval.debugger.logger.clone());
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, eval.debugger.clone());
        let prog = parser.parse_program().expect("Parsing must succeed");

        let result = eval.eval(&ASTNode::Program(prog)).expect("Eval must succeed");
        assert_eq!(
            result,
            RuntimeValue::State(QuadState::Proceed),
            "Exemplar 'A' (65 <=> [ -1, +1, +1, -1, +1 ]) must evaluate to +1:PROCEED"
        );
    }

    /// Witness Proof 2: Proverbs 24:27 KJV
    /// Prepares and proves the SVO action verbs (INSCRIBE, TRANSDUCE, PROJECT, SEQUENCE)
    /// in isolation before constructing higher-level systems.
    #[test]
    fn test_svo_verbs_character_lifecycle() {
        let mut eval = create_test_evaluator();

        // 1. Inscribe incoming glyph 'A' (65)
        eval.env.set("Glyph", RuntimeValue::Text("A".into()));
        let node_inscribe = ASTNode::Entity(EntityNode {
            name: "INSCRIBE".into(),
            definition: BoundaryNode {
                boundary_type: crate::core::boundaries::BoundaryType::Denotation,
                elements: vec![ASTNode::Identifier("Cell".into())],
                aside: None,
                line: 1,
                col: 1,
            },
            aside: None,
            context: None,
            line: 1,
            col: 1,
        });
        let res_inscribe = eval.eval(&node_inscribe).expect("INSCRIBE must succeed");
        assert!(matches!(res_inscribe, RuntimeValue::Character { glyph: 65, .. }));

        // 2. Transduce Character into Cell trits
        let node_transduce = ASTNode::Entity(EntityNode {
            name: "TRANSDUCE".into(),
            definition: BoundaryNode {
                boundary_type: crate::core::boundaries::BoundaryType::Denotation,
                elements: vec![ASTNode::Identifier("Cell`".into())],
                aside: None,
                line: 2,
                col: 1,
            },
            aside: None,
            context: None,
            line: 2,
            col: 1,
        });
        let res_transduce = eval.eval(&node_transduce).expect("TRANSDUCE must succeed");
        assert!(matches!(res_transduce, RuntimeValue::Cell(_)));

        // 3. Project Character to Target (simulating terminal stdout / INTO port)
        let node_project = ASTNode::Entity(EntityNode {
            name: "PROJECT".into(),
            definition: BoundaryNode {
                boundary_type: crate::core::boundaries::BoundaryType::Denotation,
                elements: vec![ASTNode::Identifier("Target".into())],
                aside: None,
                line: 3,
                col: 1,
            },
            aside: None,
            context: None,
            line: 3,
            col: 1,
        });
        let res_project = eval.eval(&node_project).expect("PROJECT must succeed");
        assert!(matches!(res_project, RuntimeValue::Character { glyph: 65, .. }));
        assert_eq!(eval.env.read_into(), RuntimeValue::from_glyph(65));

        // 4. Sequence Character into continuous Text ribbon
        let node_sequence = ASTNode::Entity(EntityNode {
            name: "SEQUENCE".into(),
            definition: BoundaryNode {
                boundary_type: crate::core::boundaries::BoundaryType::Denotation,
                elements: vec![ASTNode::Identifier("Text".into())],
                aside: None,
                line: 4,
                col: 1,
            },
            aside: None,
            context: None,
            line: 4,
            col: 1,
        });
        let res_seq = eval.eval(&node_sequence).expect("SEQUENCE must succeed");
        assert_eq!(res_seq, RuntimeValue::Text("A".into()));
    }

    /// Witness Proof 3: Proverbs 22:28 KJV
    /// Preserves ancient landmarks by proving that a character input from the terminal
    /// (%from / 0x03F8) flows end-to-end through the Character tape, echoes outward
    /// to the terminal screen (%into / 0x000B8000), and commits in +1:PROCEED.
    #[test]
    fn test_terminal_character_echo_end_to_end() {
        let mut eval = create_test_evaluator();

        // Simulate incoming terminal keystroke 'K' (ASCII 75) arriving on %from
        eval.env.write_from(RuntimeValue::from_glyph(b'K'));
        eval.env.set("Glyph", RuntimeValue::Text("K".into()));

        // Execute Inscribe
        let inscribe_src = "{ Glyph ] --> INSCRIBE [ Cell ] = [ Character` ]";
        let tokens1 = Lexer::new(inscribe_src, eval.debugger.logger.clone()).tokenize().unwrap();
        let prog1 = Parser::new(tokens1, eval.debugger.clone()).parse_program().unwrap();
        let res1 = eval.eval(&ASTNode::Program(prog1)).expect("Roll 1 inscribe must succeed");
        assert_eq!(res1, RuntimeValue::State(QuadState::Proceed));

        // Execute Transduce
        let transduce_src = "[ Character` { 8 Bits } ] <=> TRANSDUCE [ Cell` { 5 Trits } ]";
        let tokens2 = Lexer::new(transduce_src, eval.debugger.logger.clone()).tokenize().unwrap();
        let prog2 = Parser::new(tokens2, eval.debugger.clone()).parse_program().unwrap();
        let res2 = eval.eval(&ASTNode::Program(prog2)).expect("Roll 2 transduce must succeed");
        assert_eq!(res2, RuntimeValue::State(QuadState::Proceed));

        // Execute Project (Hardware Echo to %into / Target)
        let project_src = "[ Character` } --> PROJECT [ Target ] = [ Target` }";
        let tokens3 = Lexer::new(project_src, eval.debugger.logger.clone()).tokenize().unwrap();
        let prog3 = Parser::new(tokens3, eval.debugger.clone()).parse_program().unwrap();
        let res3 = eval.eval(&ASTNode::Program(prog3)).expect("Roll 4 project must succeed");
        assert_eq!(res3, RuntimeValue::State(QuadState::Proceed));

        // Assert that the echoed character arrived in the %into register
        let into_val = eval.env.read_into();
        if let RuntimeValue::Character { glyph, .. } = into_val {
            assert_eq!(glyph, b'K', "Echoed character glyph must exactly match terminal input 'K'");
        } else {
            panic!("Expected RuntimeValue::Character in %into, got {:?}", into_val);
        }
    }
}
