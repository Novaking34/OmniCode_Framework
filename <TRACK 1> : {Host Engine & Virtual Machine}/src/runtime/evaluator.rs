// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Evaluator Identity, Quad-State Transitions & Execution Invariants )
// ==============================================================================
//! OmniCode Runtime: Evaluator & Quad-State Engine
//!
//! Executes OmniCode AST nodes, evaluating balanced ternary logic,
//! transactional rollbacks, first-class asides, and directional vectors.

use crate::ast::nodes::*;
use crate::core::cell::Cell;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::core::ternary::QuadState;
use crate::core::vessel::Vessel;
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
                // If this is an index access into an existing Vessel in the environment: Vessel[offset]
                if let Some(RuntimeValue::Vessel(ref vessel)) = self.env.get(&entity.name).cloned() {
                    if let Some(first_el) = entity.definition.elements.first() {
                        let offset_val = self.eval(first_el)?;
                        let offset = match offset_val {
                            RuntimeValue::Number(n) => n as usize,
                            _ => 0,
                        };
                        let cell = vessel.read(offset);
                        return Ok(RuntimeValue::Cell(cell));
                    }
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

                // Check if this entity is declared as a Vessel or Storehouse
                let is_vessel_decl = entity.context.is_some()
                    || entity.name == "Storehouse"
                    || entity.name == "Vessel"
                    || entity.name == "Partition"
                    || entity.name.ends_with("_Storehouse")
                    || entity.name.ends_with("_Partition")
                    || entity.name.ends_with("_Vessel")
                    || entity.name.starts_with("Storehouse_")
                    || entity.name.starts_with("Partition_")
                    || entity.name.starts_with("Vessel_")
                    || entity.name.starts_with("Main_")
                    || entity.name.starts_with("Task_");

                if is_vessel_decl {
                    if let RuntimeValue::Number(cap) = val {
                        let is_storehouse = entity.name.contains("Storehouse");
                        let vessel = if is_storehouse {
                            Vessel::new_storehouse(&entity.name, cap as usize)
                        } else {
                            Vessel::new(&entity.name, cap as usize)
                        };
                        self.env.set(&entity.name, RuntimeValue::Vessel(vessel.clone()));
                        self.debugger.logger.log("VESSEL", format!("Allocated {vessel}"));
                        return Ok(RuntimeValue::Vessel(vessel));
                    }
                }

                self.env.set(&entity.name, val.clone());
                self.debugger.logger.log("ENTITY", format!("Defined entity '{}' = {:?}", entity.name, val));
                Ok(val)
            }

            ASTNode::Relation(rel) => {
                let left_val = self.eval(&rel.left)?;
                let right_val = self.eval(&rel.right)?;

                let result = match rel.relation {
                    BeingRelation::Is => {
                        if let (RuntimeValue::Number(n), RuntimeValue::Text(t)) = (&left_val, &right_val) {
                            let is_even = (n.round() as i64) % 2 == 0;
                            if (t.eq_ignore_ascii_case("even") && is_even) || (t.eq_ignore_ascii_case("odd") && !is_even) {
                                QuadState::Proceed
                            } else {
                                QuadState::Rollback
                            }
                        } else if left_val == right_val {
                            QuadState::Proceed
                        } else {
                            QuadState::Rollback
                        }
                    }
                    BeingRelation::IsNot => {
                        if let (RuntimeValue::Number(n), RuntimeValue::Text(t)) = (&left_val, &right_val) {
                            let is_even = (n.round() as i64) % 2 == 0;
                            if (t.eq_ignore_ascii_case("even") && !is_even) || (t.eq_ignore_ascii_case("odd") && is_even) {
                                QuadState::Proceed
                            } else {
                                QuadState::Rollback
                            }
                        } else if left_val != right_val {
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
                    BeingRelation::Equivalence => {
                        if left_val == right_val {
                            QuadState::Proceed
                        } else {
                            QuadState::Rollback
                        }
                    }
                    BeingRelation::LessThan => {
                        match (&left_val, &right_val) {
                            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                                if a < b { QuadState::Proceed } else { QuadState::Rollback }
                            }
                            _ => QuadState::Rollback,
                        }
                    }
                    BeingRelation::GreaterThan => {
                        match (&left_val, &right_val) {
                            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                                if a > b { QuadState::Proceed } else { QuadState::Rollback }
                            }
                            _ => QuadState::Rollback,
                        }
                    }
                    BeingRelation::LessEqual => {
                        match (&left_val, &right_val) {
                            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                                if a <= b { QuadState::Proceed } else { QuadState::Rollback }
                            }
                            _ => QuadState::Rollback,
                        }
                    }
                    BeingRelation::GreaterEqual => {
                        match (&left_val, &right_val) {
                            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                                if a >= b { QuadState::Proceed } else { QuadState::Rollback }
                            }
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
                    // Check if target is a Vessel indexed write: Value --> Vessel[Offset]
                    if let ASTNode::Entity(ref target_entity) = *dir.target {
                        if let Some(RuntimeValue::Vessel(mut vessel)) = self.env.get(&target_entity.name).cloned() {
                            let src_val = self.eval(&dir.source)?;
                            let offset_val = if let Some(first_el) = target_entity.definition.elements.first() {
                                self.eval(first_el)?
                            } else {
                                RuntimeValue::Number(0.0)
                            };
                            let offset = match offset_val {
                                RuntimeValue::Number(n) => n as usize,
                                _ => 0,
                            };
                            let cell = match src_val {
                                RuntimeValue::Number(n) => Cell::from_natural(n as u8).unwrap_or_else(|_| Cell::from_state(QuadState::Rollback)),
                                RuntimeValue::State(s) => Cell::from_state(s),
                                RuntimeValue::Cell(c) => c,
                                _ => Cell::from_state(QuadState::Proceed),
                            };
                            let write_res = vessel.write(offset, cell).unwrap_or(QuadState::Rollback);
                            self.env.set(&target_entity.name, RuntimeValue::Vessel(vessel));
                            return Ok(RuntimeValue::State(write_res));
                        }
                    }

                    let src_val = self.eval(&dir.source)?;
                    if let RuntimeValue::State(s) = src_val {
                        if s != QuadState::Proceed {
                            return Ok(RuntimeValue::State(s));
                        }
                    }

                    // Check if target is a recognized Silicon Register or variable assignment
                    if let ASTNode::Identifier(ref target_name) = *dir.target {
                        let is_quad_keyword = target_name.eq_ignore_ascii_case("proceed")
                            || target_name.eq_ignore_ascii_case("await")
                            || target_name.eq_ignore_ascii_case("rollback")
                            || target_name.eq_ignore_ascii_case("halt")
                            || target_name.eq_ignore_ascii_case("yes")
                            || target_name.eq_ignore_ascii_case("no");

                        if !is_quad_keyword {
                            if let Some(reg) = crate::runtime::assembler::MachineRegister::from_name(target_name) {
                                if reg == crate::runtime::assembler::MachineRegister::INTO {
                                    self.env.write_into(src_val.clone());
                                } else if reg == crate::runtime::assembler::MachineRegister::DISCARD {
                                    // Bit-bucket: safely dropped
                                } else {
                                    self.env.set(target_name, src_val.clone());
                                }
                                return Ok(src_val);
                            } else if matches!(*dir.source, ASTNode::Number(_) | ASTNode::StringLit(_) | ASTNode::Ternary(_)) {
                                self.env.set(target_name, src_val.clone());
                                return Ok(src_val);
                            }
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

                    // Check if target is a Vessel indexed read: Target <-- Vessel[Offset]
                    if let ASTNode::Entity(ref target_entity) = *dir.target {
                        if let Some(RuntimeValue::Vessel(ref vessel)) = self.env.get(&target_entity.name).cloned() {
                            let offset_val = if let Some(first_el) = target_entity.definition.elements.first() {
                                self.eval(first_el)?
                            } else {
                                RuntimeValue::Number(0.0)
                            };
                            let offset = match offset_val {
                                RuntimeValue::Number(n) => n as usize,
                                _ => 0,
                            };
                            let cell = vessel.read(offset);
                            if let ASTNode::Identifier(ref dest_name) = *dir.source {
                                self.env.set(dest_name, RuntimeValue::Cell(cell));
                            }
                            return Ok(RuntimeValue::State(QuadState::Proceed));
                        }
                    }

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
                    // Check if partitioning: ParentStorehouse v-- ChildPartition [ Size ]
                    if let ASTNode::Identifier(ref parent_name) = *dir.source {
                        if let Some(RuntimeValue::Vessel(mut parent_vessel)) = self.env.get(parent_name).cloned() {
                            if let ASTNode::Entity(ref child_entity) = *dir.target {
                                let cap_val = if let Some(first_el) = child_entity.definition.elements.first() {
                                    self.eval(first_el)?
                                } else {
                                    RuntimeValue::Number(256.0)
                                };
                                let child_cap = match cap_val {
                                    RuntimeValue::Number(n) => n as usize,
                                    _ => 256,
                                };
                                match parent_vessel.partition(&child_entity.name, child_cap) {
                                    Ok(child_part) => {
                                        self.env.set(parent_name, RuntimeValue::Vessel(parent_vessel));
                                        self.env.set(&child_entity.name, RuntimeValue::Vessel(child_part));
                                        return Ok(RuntimeValue::State(QuadState::Proceed));
                                    }
                                    Err(err) => {
                                        self.debugger.logger.failpoint(crate::diagnostics::logger::FailpointType::BoundaryLeak, "EVALUATOR", &err);
                                        return Ok(RuntimeValue::State(QuadState::Rollback));
                                    }
                                }
                            }
                        }
                    }
                    self.eval(&dir.target)
                }

                DirectionalVector::Major => {
                    self.eval(&dir.target)
                }

                DirectionalVector::Minor => {
                    self.eval(&dir.target)
                }

                DirectionalVector::Great => {
                    self.eval(&dir.source)
                }

                DirectionalVector::Small => {
                    self.eval(&dir.target)
                }

                DirectionalVector::UpperRight
                | DirectionalVector::LowerRight
                | DirectionalVector::UpperLeft
                | DirectionalVector::LowerLeft => {
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
