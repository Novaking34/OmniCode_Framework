// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Morphological Grammar: Precedence Cascade & Dual-Vector Physics )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
//
// DUAL-VECTOR ARCHITECTURE:
//   1. FLOW DOWN (Decomposition / Macrocosm to Microcosm):
//      Program -> Statement -> Caveat/Connective (L1) -> Directional (L2) ->
//      Relation (L3) -> Arithmetic (L4) -> Unary (L5) -> Primary (L6).
//
//   2. FLOW BACK UP (Synthesis / Inscription & Folding):
//      Primary atoms fold upward into binary relations, directional flows,
//      and first-class intent asides `( ... )`, anchoring into complete rolls.
// ==============================================================================

use crate::ast::nodes::*;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::parser::cursor::TokenCursor;
use crate::parser::lexer::TokenType;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Grammar Engine Context & State Mounting )
// ==============================================================================

/// Grammar engine driving recursive-descent parsing over a [`TokenCursor`].
pub struct GrammarEngine<'a> {
    /// Active token stream cursor providing lookahead and expectation guards.
    pub cursor: &'a mut TokenCursor,
}

impl<'a> GrammarEngine<'a> {
    /// Constructs a new [`GrammarEngine`] mounted over a [`TokenCursor`].
    pub fn new(cursor: &'a mut TokenCursor) -> Self {
        Self { cursor }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Morphological Grammars: Statements, Boundaries, Downward & Upward Cascades )
// ==============================================================================

impl<'a> GrammarEngine<'a> {
    // --------------------------------------------------------------------------
    // PART 3A: MACROCOSMIC PROGRAM & STATEMENT PARSING
    // --------------------------------------------------------------------------

    /// Parses an entire source reel or tape into a unified [`ProgramNode`].
    ///
    /// Iterates through sequential top-level statements until EOF, then proves
    /// that all opened boundary frames have been cleanly popped from the stack.
    pub fn parse_program(&mut self) -> Result<ProgramNode, String> {
        let mut statements = Vec::new();

        while !self.cursor.is_at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.cursor.debugger.assert_all_boundaries_closed()?;
        Ok(ProgramNode { statements, line: 1, col: 1 })
    }

    /// Parses a single statement or top-level structural declaration.
    ///
    /// Recognizes:
    /// 1. Root assignment declarations: `= [Module]: ...` and `= [Tape]: ...`
    /// 2. Section blocks: `Opening { ... }`, `Body [ ... ]`, `Closing ( Aside ) { ... }`
    /// 3. Attached body blocks: `< Character > = ... ( Aside ) { ... }`
    /// 4. Expressions, entities, and directional transitions.
    pub fn parse_statement(&mut self) -> Result<ASTNode, String> {
        // 1. Handle optional leading '=' for root module or tape declarations:
        if self.cursor.check(&TokenType::Is) {
            let eq_tok = self.cursor.advance();
            let expr = self.parse_expression()?;

            // Check if immediately followed by an attached body boundary block
            if matches!(self.cursor.peek().token_type, TokenType::LBrace | TokenType::LBracket) {
                let body_boundary = self.parse_boundary()?;
                return Ok(ASTNode::Connective(ConnectiveNode {
                    left: Box::new(expr),
                    connective: Connective::InRelationTo,
                    right: Box::new(ASTNode::Boundary(body_boundary)),
                    line: eq_tok.line,
                    col: eq_tok.col,
                }));
            }

            return Ok(expr);
        }

        // 2. Parse base expression
        let expr = self.parse_expression()?;

        // 3. Flow Back Up: Attach optional trailing Aside and Contextual/Denotational block
        let maybe_aside = if let TokenType::Aside(ref text) = self.cursor.peek().token_type {
            Some(text.clone())
        } else {
            None
        };

        if let Some(aside_text) = maybe_aside {
            if matches!(self.cursor.peek_at(1).token_type, TokenType::LBrace | TokenType::LBracket) {
                let a_tok = self.cursor.advance(); // consume Aside
                let mut body_block = self.parse_boundary()?;
                if body_block.aside.is_none() {
                    body_block.aside = Some(AsideNode {
                        intent: aside_text,
                        line: a_tok.line,
                        col: a_tok.col,
                    });
                }
                return Ok(ASTNode::Connective(ConnectiveNode {
                    left: Box::new(expr),
                    connective: Connective::InRelationTo,
                    right: Box::new(ASTNode::Boundary(body_block)),
                    line: a_tok.line,
                    col: a_tok.col,
                }));
            }
        }

        // Check if directly followed by an attached body boundary block without aside
        if matches!(self.cursor.peek().token_type, TokenType::LBrace | TokenType::LBracket) {
            let body_block = self.parse_boundary()?;
            return Ok(ASTNode::Connective(ConnectiveNode {
                left: Box::new(expr),
                connective: Connective::InRelationTo,
                right: Box::new(ASTNode::Boundary(body_block)),
                line: self.cursor.peek().line,
                col: self.cursor.peek().col,
            }));
        }

        Ok(expr)
    }

    // --------------------------------------------------------------------------
    // PART 3B: MORPHOLOGICAL BOUNDARY ENGINE (THE 6 BOUNDARY RANKS)
    // --------------------------------------------------------------------------

    /// Parses a morphological boundary block, enforcing boundary stack physics:
    ///   - Rank 1: Invariant Axiom       `< ... >`
    ///   - Rank 2: Denotation            `[ ... ]`
    ///   - Rank 3: Connotation           `{ ... }`
    ///   - Rank 4: Inside-Out Emergence  `[ ... }`
    ///   - Rank 5: Outside-In Imposition `{ ... ]`
    ///   - Rank 6: Intent Aside          `( ... )`
    pub fn parse_boundary(&mut self) -> Result<BoundaryNode, String> {
        let (_open_char, open_tok) = self.cursor.expect_boundary_open("BoundaryBlock")?;
        let mut elements = Vec::new();

        while !self.cursor.is_at_end() && !matches!(
            self.cursor.peek().token_type,
            TokenType::RBracket | TokenType::RBrace | TokenType::RAngle
        ) {
            let el = self.parse_expression()?;
            elements.push(el);
        }

        let (btype, _close_tok) = self.cursor.expect_boundary_close("BoundaryBlock")?;

        // Flow Back Up: Check for attached first-class Aside: ( ... )
        let mut aside = None;
        let maybe_aside = if let TokenType::Aside(ref text) = self.cursor.peek().token_type {
            Some(text.clone())
        } else {
            None
        };
        if let Some(text) = maybe_aside {
            let a_tok = self.cursor.advance();
            aside = Some(AsideNode {
                intent: text,
                line: a_tok.line,
                col: a_tok.col,
            });
        }

        Ok(BoundaryNode {
            boundary_type: btype,
            elements,
            aside,
            line: open_tok.line,
            col: open_tok.col,
        })
    }

    // --------------------------------------------------------------------------
    // PART 3C: DOWNWARD PRECEDENCE CASCADE (FLOW DOWN)
    // --------------------------------------------------------------------------

    /// Entry point for expression hierarchy parsing.
    #[inline]
    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_caveat_or_connective()
    }

    /// Precedence Level 1: Adversative Caveat `!` (BUT / Guard Interlock)
    /// and Connectives `::` / `&` (And), `|` (Or), `:` / `@` / `::@` (In Relation To / Located At).
    pub fn parse_caveat_or_connective(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_directional()?;

        while matches!(
            self.cursor.peek().token_type,
            TokenType::But | TokenType::And | TokenType::HardwareLocator | TokenType::Or | TokenType::InRelationTo | TokenType::BoundPair | TokenType::At
        ) {
            let op_tok = self.cursor.advance();
            let connective = match op_tok.token_type {
                TokenType::But => Connective::But,
                TokenType::And | TokenType::BoundPair | TokenType::HardwareLocator => Connective::And,
                TokenType::Or => Connective::Or,
                TokenType::InRelationTo | TokenType::At => Connective::InRelationTo,
                _ => unreachable!(),
            };
            let right = self.parse_directional()?;
            expr = ASTNode::Connective(ConnectiveNode {
                left: Box::new(expr),
                connective,
                right: Box::new(right),
                line: op_tok.line,
                col: op_tok.col,
            });
        }

        Ok(expr)
    }

    /// Precedence Level 2: Directional Vectors across X, Y, and Z axes:
    ///   `-->` (ActUpon), `<--` (ActAgain), `^--` (BuildsTo), `v--` (BreaksDownFrom),
    ///   as well as scale modulations and quadrant vectors.
    pub fn parse_directional(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_relation()?;

        while matches!(
            self.cursor.peek().token_type,
            TokenType::ActUpon
                | TokenType::ActAgain
                | TokenType::BuildsTo
                | TokenType::BreaksDownFrom
                | TokenType::Major
                | TokenType::Minor
                | TokenType::Great
                | TokenType::Small
                | TokenType::UpperRight
                | TokenType::LowerRight
                | TokenType::UpperLeft
                | TokenType::LowerLeft
        ) {
            let op_tok = self.cursor.advance();
            let vector = match op_tok.token_type {
                TokenType::ActUpon => DirectionalVector::ActUpon,
                TokenType::ActAgain => DirectionalVector::ActAgain,
                TokenType::BuildsTo => DirectionalVector::BuildsTo,
                TokenType::BreaksDownFrom => DirectionalVector::BreaksDownFrom,
                TokenType::Major => DirectionalVector::Major,
                TokenType::Minor => DirectionalVector::Minor,
                TokenType::Great => DirectionalVector::Great,
                TokenType::Small => DirectionalVector::Small,
                TokenType::UpperRight => DirectionalVector::UpperRight,
                TokenType::LowerRight => DirectionalVector::LowerRight,
                TokenType::UpperLeft => DirectionalVector::UpperLeft,
                TokenType::LowerLeft => DirectionalVector::LowerLeft,
                _ => unreachable!(),
            };
            let target = self.parse_relation()?;
            expr = ASTNode::Directional(DirectionalNode {
                source: Box::new(expr),
                vector,
                target: Box::new(target),
                line: op_tok.line,
                col: op_tok.col,
            });
        }

        Ok(expr)
    }

    /// Precedence Level 3: Being Relations and Invariant Equivalence:
    ///   `=` (Is), `!=` (IsNot), `~=` (ToBeLike), `<=>` (InvariantEquivalence / Transduction),
    ///   `<`, `>`, `<=`, `>=` (Relational Bound Comparisons).
    pub fn parse_relation(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_arithmetic()?;

        while matches!(
            self.cursor.peek().token_type,
            TokenType::Is
                | TokenType::IsNot
                | TokenType::ToBeLike
                | TokenType::InvariantEquivalence
                | TokenType::LessThan
                | TokenType::GreaterThan
                | TokenType::LessEqual
                | TokenType::GreaterEqual
        ) {
            let op_tok = self.cursor.advance();
            let relation = match op_tok.token_type {
                TokenType::Is => BeingRelation::Is,
                TokenType::IsNot => BeingRelation::IsNot,
                TokenType::ToBeLike => BeingRelation::ToBeLike,
                TokenType::InvariantEquivalence => BeingRelation::Equivalence,
                TokenType::LessThan => BeingRelation::LessThan,
                TokenType::GreaterThan => BeingRelation::GreaterThan,
                TokenType::LessEqual => BeingRelation::LessEqual,
                TokenType::GreaterEqual => BeingRelation::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_arithmetic()?;
            expr = ASTNode::Relation(RelationNode {
                left: Box::new(expr),
                relation,
                right: Box::new(right),
                line: op_tok.line,
                col: op_tok.col,
            });
        }

        Ok(expr)
    }

    /// Precedence Level 4: SVO Arithmetic Math Keywords:
    ///   `plus`, `minus`, `times`, `div`.
    pub fn parse_arithmetic(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_unary()?;

        while matches!(
            self.cursor.peek().token_type,
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Div
        ) {
            let op_tok = self.cursor.advance();
            let op = match op_tok.token_type {
                TokenType::Plus => ArithmeticOp::Add,
                TokenType::Minus => ArithmeticOp::Sub,
                TokenType::Times => ArithmeticOp::Mul,
                TokenType::Div => ArithmeticOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = ASTNode::Arithmetic(ArithmeticNode {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                line: op_tok.line,
                col: op_tok.col,
            });
        }

        Ok(expr)
    }

    /// Precedence Level 5: Unary Operations:
    ///   `--` (Not) and Prefix Vectors (`-->`, `<--`, `^--`, `v--`).
    pub fn parse_unary(&mut self) -> Result<ASTNode, String> {
        if self.cursor.check(&TokenType::Not) {
            let op_tok = self.cursor.advance();
            let operand = self.parse_unary()?;
            return Ok(ASTNode::UnaryNot(UnaryNotNode {
                operand: Box::new(operand),
                line: op_tok.line,
                col: op_tok.col,
            }));
        }

        if matches!(
            self.cursor.peek().token_type,
            TokenType::ActAgain | TokenType::ActUpon | TokenType::BuildsTo | TokenType::BreaksDownFrom
        ) {
            let op_tok = self.cursor.advance();
            let vector = match op_tok.token_type {
                TokenType::ActAgain => DirectionalVector::ActAgain,
                TokenType::ActUpon => DirectionalVector::ActUpon,
                TokenType::BuildsTo => DirectionalVector::BuildsTo,
                TokenType::BreaksDownFrom => DirectionalVector::BreaksDownFrom,
                _ => unreachable!(),
            };
            let operand = self.parse_unary()?;
            return Ok(ASTNode::Directional(DirectionalNode {
                source: Box::new(ASTNode::Ternary(crate::core::ternary::QuadState::Proceed)),
                vector,
                target: Box::new(operand),
                line: op_tok.line,
                col: op_tok.col,
            }));
        }

        self.parse_primary()
    }

    // --------------------------------------------------------------------------
    // PART 3D: UPWARD INSCRIPTION & SYNTHESIS (FLOW BACK UP)
    // --------------------------------------------------------------------------

    /// Precedence Level 6: Primaries & Atom Synthesis:
    ///   Morphological Boundaries, Affixed Polarities, Boundary Casts, Asides,
    ///   Ternary Literals, Numbers, Strings, Identifiers (with state-primes `),
    ///   and Complex Entities with Definitions, Asides, and Contexts.
    pub fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let tok = self.cursor.peek().clone();
        match tok.token_type {
            TokenType::LBracket | TokenType::LBrace | TokenType::LAngle => {
                let b = self.parse_boundary()?;
                Ok(ASTNode::Boundary(b))
            }
            TokenType::AffixedIdent { ref name, polarity, position } => {
                let id_name = name.clone();
                let line = tok.line;
                let col = tok.col;
                self.cursor.advance();
                Ok(ASTNode::Polarity(PolarityNode {
                    name: id_name,
                    polarity,
                    position,
                    line,
                    col,
                }))
            }
            TokenType::BoundaryCast { boundary, polarity } => {
                let line = tok.line;
                let col = tok.col;
                self.cursor.advance();
                Ok(ASTNode::BoundaryCast(BoundaryCastNode {
                    boundary,
                    polarity,
                    line,
                    col,
                }))
            }
            TokenType::Aside(ref intent) => {
                self.cursor.advance();
                Ok(ASTNode::Aside(AsideNode {
                    intent: intent.clone(),
                    line: tok.line,
                    col: tok.col,
                }))
            }
            TokenType::Ternary(state) => {
                self.cursor.advance();
                Ok(ASTNode::Ternary(state))
            }
            TokenType::Number(val) => {
                self.cursor.advance();
                Ok(ASTNode::Number(val))
            }
            TokenType::StringLit(ref s) => {
                self.cursor.advance();
                Ok(ASTNode::StringLit(s.clone()))
            }
            TokenType::CharLit(c) => {
                let s = c.to_string();
                self.cursor.advance();
                Ok(ASTNode::StringLit(s))
            }
            TokenType::Infer => {
                self.cursor.advance();
                Ok(ASTNode::Identifier("?".into()))
            }
            TokenType::Pronoun(ref p) => {
                let name = p.as_str().to_string();
                self.cursor.advance();
                Ok(ASTNode::Identifier(name))
            }
            TokenType::Identifier(_)
            | TokenType::Who
            | TokenType::What
            | TokenType::When
            | TokenType::Where
            | TokenType::Why
            | TokenType::How => {
                let id_str = match tok.token_type {
                    TokenType::Identifier(ref n) => n.clone(),
                    _ => tok.lexeme.clone(),
                };
                let next_tok = self.cursor.peek_at(1).token_type.clone();

                // 1. Entity Syntax: Name [ Definition ] ( Aside ) { Context }
                if matches!(next_tok, TokenType::LBracket | TokenType::LAngle | TokenType::LBrace) {
                    let id_name = id_str;
                    let line = tok.line;
                    let col = tok.col;
                    self.cursor.advance(); // consume identifier

                    let def_node = self.parse_boundary()?;
                    let mut aside_node = None;
                    let mut context_node = None;

                    // Check for optional Aside: ( ... )
                    let maybe_aside = if let TokenType::Aside(ref text) = self.cursor.peek().token_type {
                        Some(text.clone())
                    } else {
                        None
                    };
                    if let Some(text) = maybe_aside {
                        let a_tok = self.cursor.advance();
                        aside_node = Some(AsideNode {
                            intent: text,
                            line: a_tok.line,
                            col: a_tok.col,
                        });
                    }

                    // Check for optional Context boundary
                    if matches!(self.cursor.peek().token_type, TokenType::LBrace | TokenType::LBracket) {
                        context_node = Some(self.parse_boundary()?);
                    }

                    if def_node.boundary_type.is_immutable() {
                        self.cursor.debugger.register_immutable(&id_name);
                    }

                    return Ok(ASTNode::Entity(EntityNode {
                        name: id_name,
                        definition: def_node,
                        aside: aside_node,
                        context: context_node,
                        line,
                        col,
                    }));
                }

                // 2. Entity Syntax: Name ( Aside ) { Context }
                if let TokenType::Aside(ref text) = next_tok {
                    if matches!(self.cursor.peek_at(2).token_type, TokenType::LBrace | TokenType::LBracket | TokenType::LAngle) {
                        let id_name = id_str;
                        let line = tok.line;
                        let col = tok.col;
                        let aside_text = text.clone();
                        self.cursor.advance(); // consume identifier
                        let a_tok = self.cursor.advance(); // consume aside
                        let def_node = self.parse_boundary()?;

                        return Ok(ASTNode::Entity(EntityNode {
                            name: id_name,
                            definition: def_node,
                            aside: Some(AsideNode {
                                intent: aside_text,
                                line: a_tok.line,
                                col: a_tok.col,
                            }),
                            context: None,
                            line,
                            col,
                        }));
                    }
                }

                self.cursor.advance();
                Ok(ASTNode::Identifier(id_str))
            }
            _ => {
                let msg = format!("Unexpected token '{:?}' in expression at L{}:C{}", tok.token_type, tok.line, tok.col);
                self.cursor.debugger.logger.failpoint(crate::diagnostics::logger::FailpointType::SyntaxAnomaly, "GRAMMAR", &msg);
                Err(format!("FAILPOINT: {msg}"))
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Grammar Verification, Boundary Integrity Proofs & Unit Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::debugger::OmniDebugger;
    use crate::diagnostics::logger::DiagnosticLogger;

    fn parse_src(src: &str) -> Result<ProgramNode, String> {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut cursor = TokenCursor::new(tokens, debugger);
        let mut engine = GrammarEngine::new(&mut cursor);
        engine.parse_program()
    }

    #[test]
    fn test_grammar_precedence_downward_and_upward() {
        let prog = parse_src("1 plus 2 times 3 <=> 7").expect("Parse expression");
        assert_eq!(prog.statements.len(), 1);
    }

    #[test]
    fn test_grammar_directional_and_connective() {
        let prog = parse_src("{ Inflow ] --> INSCRIBE [ Cell ] = [ Character` ]").expect("Parse inscribe roll");
        assert_eq!(prog.statements.len(), 1);
    }
}
