//! OmniCode Runtime: Evaluator & Quad-State Engine
//!
//! Executes OmniCode AST nodes, evaluating balanced ternary logic,
//! transactional rollbacks, first-class asides, and directional vectors.

use crate::ast::nodes::*;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::FailpointType;
use crate::runtime::environment::{Environment, RuntimeValue};

pub struct Evaluator {
    pub env: Environment,
    pub debugger: OmniDebugger,
}

impl Evaluator {
    pub fn new(env: Environment, debugger: OmniDebugger) -> Self {
        Self { env, debugger }
    }

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
                    let msg = format!("Unbound identifier '{name}'");
                    self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "EVALUATOR", &msg);
                    Err(format!("FAILPOINT: {msg}"))
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
                        // Structural analogy: matches if both are same category
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
                    // Action ! Caveat: Begin transaction on action, verify caveat
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
                    // Projection: evaluates right in relation to left
                    self.eval(&conn.right)
                }
            },

            ASTNode::Directional(dir) => match dir.vector {
                DirectionalVector::ActUpon => {
                    // Forward causation: source --> target
                    let src_val = self.eval(&dir.source)?;
                    if let RuntimeValue::State(s) = src_val {
                        if s != QuadState::Proceed {
                            return Ok(RuntimeValue::State(s));
                        }
                    }
                    self.eval(&dir.target)
                }

                DirectionalVector::ActAgain => {
                    // Backward retroaction / retry: source <-- target
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
                    // Layering Up: evaluates abstraction
                    self.eval(&dir.target)
                }

                DirectionalVector::BreaksDownFrom => {
                    // Layering Down: evaluates decomposition
                    self.eval(&dir.target)
                }
            },

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
}
