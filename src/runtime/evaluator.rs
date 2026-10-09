// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Evaluator Identity, Quad-State Transitions & Execution Invariants )
// ==============================================================================
//! OmniCode Runtime: Evaluator & Quad-State Engine
//!
//! Executes OmniCode AST nodes, evaluating balanced ternary logic,
//! transactional rollbacks, first-class asides, and directional vectors.

use crate::ast::nodes::*;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Evaluator State Initialization and Context Setup )
// ==============================================================================
pub struct Evaluator {
    pub env: Environment,
    pub debugger: OmniDebugger,
    pub loaded_modules: std::collections::HashSet<String>,
}

impl Evaluator {
    pub fn new(env: Environment, debugger: OmniDebugger) -> Self {
        Self {
            env,
            debugger,
            loaded_modules: std::collections::HashSet::new(),
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Evaluation Pipeline: Entities, Boundaries, Vectors, Ternary States )
// ==============================================================================
impl Evaluator {
    pub fn eval(&mut self, node: &ASTNode) -> Result<RuntimeValue, String> {
        match node {
            ASTNode::Ternary(state) => {
                self.debugger.inspect_state(*state, "TernaryLiteral");
                Ok(RuntimeValue::State(*state))
            }

            ASTNode::Number(val) => Ok(RuntimeValue::Number(*val)),

            ASTNode::StringLit(s) => Ok(RuntimeValue::Text(s.clone())),

            ASTNode::Identifier(name) => {
                if let Some(val) = self.env.get(name) {
                    Ok(val.clone())
                } else {
                    self.debugger.logger.log("IDENTIFIER", format!("Symbolic ontological reference: '{name}'"));
                    Ok(RuntimeValue::Text(name.clone()))
                }
            }

            ASTNode::Aside(aside) => {
                self.debugger.logger.log("ASIDE", format!("Evaluating Aside context: '{}'", aside.intent));
                Ok(RuntimeValue::State(QuadState::Proceed))
            }

            ASTNode::Boundary(b) => {
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

            ASTNode::Entity(entity) => {
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

            ASTNode::Relation(rel) => {
                let left_val = self.eval(&rel.left)?;
                let right_val = self.eval(&rel.right)?;

                let result = match rel.relation {
                    BeingRelation::Is => {
                        if left_val == right_val {
                            QuadState::Proceed
                        } else {
                            QuadState::Rollback
                        }
                    }
                    BeingRelation::IsNot => {
                        if left_val != right_val {
                            QuadState::Proceed
                        } else {
                            QuadState::Rollback
                        }
                    }
                    BeingRelation::ToBeLike => {
                        match (&left_val, &right_val) {
                            (RuntimeValue::Number(_), RuntimeValue::Number(_)) => QuadState::Proceed,
                            (RuntimeValue::Text(_), RuntimeValue::Text(_)) => QuadState::Proceed,
                            (RuntimeValue::State(_), RuntimeValue::State(_)) => QuadState::Proceed,
                            _ => QuadState::Rollback,
                        }
                    }
                };

                self.debugger.inspect_state(result, "BeingRelation");
                Ok(RuntimeValue::State(result))
            }

            ASTNode::Connective(conn) => match conn.connective {
                Connective::But => {
                    self.env.begin_transaction();
                    let action_res = self.eval(&conn.left)?;
                    let caveat_res = self.eval(&conn.right)?;

                    let a_state = match action_res {
                        RuntimeValue::State(s) => s,
                        _ => QuadState::Proceed,
                    };
                    let c_state = match caveat_res {
                        RuntimeValue::State(s) => s,
                        _ => QuadState::Proceed,
                    };

                    let final_state = a_state.but(c_state);

                    if final_state == QuadState::Rollback {
                        self.env.rollback_transaction()?;
                        self.debugger.logger.failpoint(
                            FailpointType::CaveatVeto,
                            "EVALUATOR",
                            "Action vetoed by ! BUT caveat; state transaction rolled back",
                        );
                    } else if final_state == QuadState::Halt {
                        self.env.rollback_transaction()?;
                        self.debugger.logger.failpoint(
                            FailpointType::NullTrap,
                            "EVALUATOR",
                            "Action aborted by NULL caveat",
                        );
                    } else {
                        self.env.commit_transaction();
                    }

                    Ok(RuntimeValue::State(final_state))
                }

                Connective::And => {
                    let l = self.eval_to_state(&conn.left)?;
                    let r = self.eval_to_state(&conn.right)?;
                    Ok(RuntimeValue::State(l.and(r)))
                }

                Connective::Or => {
                    let l = self.eval_to_state(&conn.left)?;
                    let r = self.eval_to_state(&conn.right)?;
                    Ok(RuntimeValue::State(l.or(r)))
                }

                Connective::Not => {
                    let r = self.eval_to_state(&conn.right)?;
                    Ok(RuntimeValue::State(r.invert()))
                }

                Connective::InRelationTo => {
                    if let ASTNode::Identifier(ref name) = *conn.left {
                        if name == "import" {
                            let mut mod_names = Vec::new();
                            self.extract_module_names(&conn.right, &mut mod_names);
                            for m in mod_names {
                                self.import_module(&m)?;
                            }
                            return Ok(RuntimeValue::State(QuadState::Proceed));
                        }
                    }
                    let _ = self.eval(&conn.right)?;
                    self.eval(&conn.left)
                }
            },

            ASTNode::Directional(dir) => match dir.vector {
                DirectionalVector::ActUpon => {
                    let src_val = self.eval(&dir.source)?;
                    if let RuntimeValue::State(s) = src_val {
                        if s != QuadState::Proceed {
                            return Ok(RuntimeValue::State(s));
                        }
                    }
                    let target_val = self.eval(&dir.target)?;
                    if let ASTNode::Identifier(ref name) = *dir.source {
                        self.debugger.assert_mutation_permitted(name)?;
                        self.env.set(name, target_val.clone());
                    }
                    Ok(target_val)
                }

                DirectionalVector::ActAgain => {
                    self.debugger.track_retroaction("ActAgain")?;
                    let target_val = self.eval(&dir.target)?;
                    if let RuntimeValue::State(s) = target_val {
                        if s == QuadState::Rollback {
                            self.env.rollback_transaction().ok();
                            return Ok(RuntimeValue::State(QuadState::Rollback));
                        }
                    }
                    self.eval(&dir.source)
                }

                DirectionalVector::BuildsTo => {
                    self.eval(&dir.target)
                }

                DirectionalVector::BreaksDownFrom => {
                    self.eval(&dir.target)
                }
            },

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

            ASTNode::UnaryNot(un) => {
                let s = self.eval_to_state(&un.operand)?;
                Ok(RuntimeValue::State(s.invert()))
            }

            ASTNode::Program(prog) => {
                let mut last = RuntimeValue::State(QuadState::Proceed);
                for stmt in &prog.statements {
                    last = self.eval(stmt)?;
                }
                Ok(last)
            }
        }
    }

    fn eval_to_state(&mut self, node: &ASTNode) -> Result<QuadState, String> {
        let val = self.eval(node)?;
        match val {
            RuntimeValue::State(s) => Ok(s),
            _ => Ok(QuadState::Proceed),
        }
    }

    fn extract_module_names(&self, node: &ASTNode, names: &mut Vec<String>) {
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

    pub fn resolve_module_path(&self, module_name: &str) -> Option<std::path::PathBuf> {
        let parts: Vec<&str> = module_name.split('.').collect();
        let rel_slash = parts.join("/").to_lowercase();
        let rel_dot = module_name.to_lowercase();

        let candidates = [
            format!("lib/{rel_slash}.omni"),
            format!("lib/{rel_dot}.omni"),
            format!("{rel_slash}.omni"),
            format!("{rel_dot}.omni"),
            format!("examples/{rel_slash}.omni"),
            format!("examples/{rel_dot}.omni"),
        ];

        for cand in &candidates {
            let path = std::path::PathBuf::from(cand);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

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

        // Evaluate definitions into environment
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
// ( Execution Invariant Proofs and Teardown )
// ==============================================================================
// Evaluator execution pipeline verified and sealed.
