// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Parser Identity, Ontological Architecture & Boundary Physics Invariants )
// ==============================================================================
//! # OmniCode Recursive Descent Parser (`parser.rs`)
//!
//! Converts raw token streams produced by the Lexer into a high-fidelity
//! Abstract Syntax Tree (AST), strictly enforcing the ontological laws,
//! directional physics, and delimiter hierarchies of OmniCode.
//!
//! ## Architectural Foundations:
//! 1. **Dual Nature: Ontological Assembly & High-Level Scripting Language**:
//!    OmniCode operates simultaneously as a bare-metal assembly language over balanced-
//!    ternary virtual machines and physical silicon registers (`%self`, `%caller`, `%acc`),
//!    and as an expressive, high-level scripting language. Verbs defined in Block 1
//!    of the Lexicon (`INSCRIBE`, `TRANSDUCE`, `GUARD`, `PROJECT`, `SEQUENCE`, `DISPATCH`)
//!    act as operational vectors in the machine and emerge as active keywords in the grammar.
//!    The Lexicon *is* the grammar—defining a type extends the compiler's active vocabulary.
//!
//! 2. **The 6 Ranked Morphological Boundaries**:
//!    - **Rank 1 `< ... >` Invariant Axiom**:
//!      Immutable law, universal truth, and absolute type definition. Cannot be mutated.
//!    - **Rank 2 `[ ... ]` Denotation**:
//!      Concrete boundary, state container, memory cell, or silicon evaluation block.
//!    - **Rank 3 `{ ... }` Connotation**:
//!      Context, environment, attribute payload, or ambient state.
//!    - **Rank 4 `[ ... }` Inside-Out Emergence (Output)**:
//!      Asymmetric boundary representing an internal entity emerging outward into
//!      the ambient universe (e.g. projecting character glyphs to terminal stdout).
//!    - **Rank 5 `{ ... ]` Outside-In Imposition (Input)**:
//!      Asymmetric boundary representing the ambient universe imposing constraints or
//!      delivering external input into a discrete entity (e.g. capturing keyboard stream).
//!    - **Rank 6 `( ... )` Intent Aside**:
//!      First-class machine teleology, architectural reasoning, and docstrings.
//!      Unlike traditional comments, Asides are preserved as permanent nodes in the AST.
//!
//! 3. **Causal State Advancement & The State-Prime (``` ` ```)**:
//!    State transitions are discrete, deterministic turns ($S \rightarrow S'$).
//!    The state-prime operator (``` ` ```) designates causal advancement without
//!    colliding with string literal delimiters (`Character``, `Cell``, `Status``).
//!
//! 4. **Hardware Locators & Composed Connectives (`::@`)**:
//!    Compound operators merge morphemic primitives: `::` (inseparable conjunction) +
//!    `@` (located at) forms `::@` ("and is located at"), binding memory-mapped I/O
//!    ports and hardware streams (e.g. `::@ 0x01` for stdout, `::@ 0x000B8000` for VGA).
//!
//! 5. **Adversative Caveat Interlocks (`!`)**:
//!    "A BUT B" — asserts an operational state transition, but catches and vetoes
//!    execution if a guarded condition trips (e.g. `! GUARD --> [ Status` = +1:PROCEED ]`).
//!
//! 6. **Operator Precedence Cascade**:
//!    - Level 1: Caveats (`!`) & Connectives (`::`, `&`, `|`, `:`, `@`, `::@`)
//!    - Level 2: Directional Vectors (`-->`, `<--`, `^--`, `v--`, scale/quadrant vectors)
//!    - Level 3: Being Relations & Transduction (`=`, `!=`, `~=`, `<=>`, `<`, `>`, `<=`, `>=`)
//!    - Level 4: SVO Arithmetic Keywords (`plus`, `minus`, `times`, `div`)
//!    - Level 5: Unary Operators (`--` Negation, prefix vectors)
//!    - Level 6: Primaries (Boundaries, Entities, Polarities, Numbers, Strings, Identifiers)

use crate::ast::nodes::*;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::{Token, TokenType};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Parser State Initialization, Token Stream Context & Navigation Helpers )
// ==============================================================================
/// State container for the recursive-descent parsing pipeline.
pub struct Parser {
    /// Ingested token stream from the Lexer.
    tokens: Vec<Token>,
    /// Current cursor index within the token stream.
    pos: usize,
    /// Active debugger enforcing boundary stack physics, delimiter integrity, and failpoints.
    pub debugger: OmniDebugger,
}

impl Parser {
    /// Constructs a new [`Parser`] over an ingested token stream.
    ///
    /// # Arguments
    /// * `tokens` - Complete token stream terminated by [`TokenType::Eof`].
    /// * `debugger` - Diagnostic debugger enforcing delimiter integrity.
    pub fn new(tokens: Vec<Token>, debugger: OmniDebugger) -> Self {
        Self {
            tokens,
            pos: 0,
            debugger,
        }
    }

    /// Inspects the current token at the cursor without advancing.
    #[inline]
    pub fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            self.tokens.last().unwrap()
        }
    }

    /// Inspects a future token at `offset` positions ahead of the cursor.
    #[inline]
    pub fn peek_at(&self, offset: usize) -> &Token {
        let target_idx = self.pos + offset;
        if target_idx < self.tokens.len() {
            &self.tokens[target_idx]
        } else {
            self.tokens.last().unwrap()
        }
    }

    /// Advances the cursor by one token and returns the consumed token.
    #[inline]
    pub fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    /// Checks if the current token matches the expected token type without consuming it.
    #[inline]
    pub fn check(&self, expected: &TokenType) -> bool {
        &self.peek().token_type == expected
    }

    /// Checks if a lookahead token at `offset` matches the expected token type.
    #[inline]
    pub fn check_at(&self, offset: usize, expected: &TokenType) -> bool {
        &self.peek_at(offset).token_type == expected
    }

    /// Conditionally consumes the current token if it matches the expected type.
    pub fn match_token(&mut self, expected: &TokenType) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Returns `true` if the cursor has reached the end-of-file token.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.check(&TokenType::Eof)
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Recursive-Descent Grammars: Program, Statements, Boundaries, Expressions )
// ==============================================================================
impl Parser {
    /// Parses an entire source reel or tape into a unified [`ProgramNode`].
    ///
    /// Iterates through sequential top-level statements until [`TokenType::Eof`],
    /// then verifies via the debugger that all opened boundaries have been cleanly closed.
    pub fn parse_program(&mut self) -> Result<ProgramNode, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.debugger.assert_all_boundaries_closed()?;
        Ok(ProgramNode { statements, line: 1, col: 1 })
    }

    /// Parses a single statement or top-level structural declaration.
    ///
    /// Supports:
    /// - Root assignment declarations: `= [Module]: ...` and `= [Tape]: ...`
    /// - 4-Block sections: `Opening { ... }`, `Body [ ... ]`, `Closing ( Aside ) { ... }`
    /// - Attached body boundary blocks: `< Character > = ... ( Aside ) { ... }`
    /// - Standard expressions, entity definitions, and relational bindings.
    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        // 1. Handle optional leading '=' for root module or tape declarations:
        //    = [Module]: < Category > & [ Noun ] ( Aside ) { ... }
        //    = [Tape]: < Category > & [ Process ] ( Aside ) { ... }
        if self.check(&TokenType::Is) {
            let eq_tok = self.advance(); // consume leading '='
            let expr = self.parse_expression()?;

            // Check if immediately followed by an attached body boundary block
            if matches!(self.peek().token_type, TokenType::LBrace | TokenType::LBracket) {
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

        // 2. Parse primary expression or entity statement
        let expr = self.parse_expression()?;

        // 3. Check for an optional trailing Aside and attached Contextual/Denotational block
        //    Examples:
        //      Closing ( Sealing Gate ... ) { ... }
        //      < Character > = < Idea > & [ Noun ] ( Aside ) { ... }
        //      Body [ ROLL 1 : ... ]
        let maybe_aside = if let TokenType::Aside(ref text) = self.peek().token_type {
            Some(text.clone())
        } else {
            None
        };

        if let Some(aside_text) = maybe_aside {
            // Peek past the Aside to see if a body boundary block follows
            if matches!(self.peek_at(1).token_type, TokenType::LBrace | TokenType::LBracket) {
                let a_tok = self.advance(); // consume Aside
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
        if matches!(self.peek().token_type, TokenType::LBrace | TokenType::LBracket) {
            let body_block = self.parse_boundary()?;
            return Ok(ASTNode::Connective(ConnectiveNode {
                left: Box::new(expr),
                connective: Connective::InRelationTo,
                right: Box::new(ASTNode::Boundary(body_block)),
                line: self.peek().line,
                col: self.peek().col,
            }));
        }

        Ok(expr)
    }

    /// Parses any morphological boundary block, enforcing the 6 boundary ranks:
    ///   - Rank 1: Invariant Axiom       `< ... >`
    ///   - Rank 2: Denotation            `[ ... ]`
    ///   - Rank 3: Connotation           `{ ... }`
    ///   - Rank 4: Inside-Out Emergence  `[ ... }` (Output projection)
    ///   - Rank 5: Outside-In Imposition `{ ... ]` (Input imposition)
    ///
    /// Also captures any first-class Aside `( ... )` attached to the closing delimiter.
    fn parse_boundary(&mut self) -> Result<BoundaryNode, String> {
        let tok = self.advance();
        let open_char = match tok.token_type {
            TokenType::LBracket => '[',
            TokenType::LBrace => '{',
            TokenType::LAngle => '<',
            _ => {
                let msg = format!("Expected boundary opening delimiter, found {:?}", tok.token_type);
                self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "PARSER", &msg);
                return Err(format!("FAILPOINT: {msg}"));
            }
        };

        self.debugger.push_boundary(open_char, tok.line, tok.col);
        let mut elements = Vec::new();

        while !self.is_at_end() && !matches!(
            self.peek().token_type,
            TokenType::RBracket | TokenType::RBrace | TokenType::RAngle
        ) {
            let el = self.parse_expression()?;
            elements.push(el);
        }

        if self.is_at_end() {
            let msg = format!("Unclosed boundary opened at L{}:C{} reaches EOF", tok.line, tok.col);
            self.debugger.logger.failpoint(FailpointType::BoundaryLeak, "PARSER", &msg);
            return Err(format!("FAILPOINT: {msg}"));
        }

        let close_tok = self.advance();
        let close_char = match close_tok.token_type {
            TokenType::RBracket => ']',
            TokenType::RBrace => '}',
            TokenType::RAngle => '>',
            _ => unreachable!(),
        };

        let btype = self.debugger.pop_boundary(close_char, close_tok.line, close_tok.col)?;

        // Check for attached first-class Aside: ( ... )
        let mut aside = None;
        let maybe_aside = if let TokenType::Aside(ref text) = self.peek().token_type {
            Some(text.clone())
        } else {
            None
        };
        if let Some(text) = maybe_aside {
            let a_tok = self.advance();
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
            line: tok.line,
            col: tok.col,
        })
    }

    /// Entry point for expression hierarchy parsing.
    #[inline]
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_caveat_or_connective()
    }

    /// Precedence Level 1: Adversative Caveat `!` (BUT / Guard Interlock)
    /// and Connectives `::` / `&` (And), `|` (Or), `:` / `@` / `::@` (In Relation To / Located At).
    fn parse_caveat_or_connective(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_directional()?;

        while matches!(
            self.peek().token_type,
            TokenType::But | TokenType::And | TokenType::Or | TokenType::InRelationTo | TokenType::BoundPair | TokenType::At
        ) {
            let op_tok = self.advance();
            let connective = match op_tok.token_type {
                TokenType::But => Connective::But,
                TokenType::And | TokenType::BoundPair => Connective::And,
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
    ///   as well as diagonal scale modulations and 2D quadrant vectors.
    fn parse_directional(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_relation()?;

        while matches!(
            self.peek().token_type,
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
            let op_tok = self.advance();
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
    fn parse_relation(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_arithmetic()?;

        while matches!(
            self.peek().token_type,
            TokenType::Is
                | TokenType::IsNot
                | TokenType::ToBeLike
                | TokenType::InvariantEquivalence
                | TokenType::LessThan
                | TokenType::GreaterThan
                | TokenType::LessEqual
                | TokenType::GreaterEqual
        ) {
            let op_tok = self.advance();
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
    fn parse_arithmetic(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_unary()?;

        while matches!(
            self.peek().token_type,
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Div
        ) {
            let op_tok = self.advance();
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
    fn parse_unary(&mut self) -> Result<ASTNode, String> {
        if self.check(&TokenType::Not) {
            let op_tok = self.advance();
            let operand = self.parse_unary()?;
            return Ok(ASTNode::UnaryNot(UnaryNotNode {
                operand: Box::new(operand),
                line: op_tok.line,
                col: op_tok.col,
            }));
        }

        if matches!(
            self.peek().token_type,
            TokenType::ActAgain | TokenType::ActUpon | TokenType::BuildsTo | TokenType::BreaksDownFrom
        ) {
            let op_tok = self.advance();
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

    /// Precedence Level 6: Primaries:
    ///   Morphological Boundaries, Affixed Polarities, Boundary Casts, Asides,
    ///   Ternary Literals, Numbers, Strings, Identifiers (with state-primes `),
    ///   and Complex Entities with Definitions, Asides, and Contexts.
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let tok = self.peek().clone();
        match tok.token_type {
            TokenType::LBracket | TokenType::LBrace | TokenType::LAngle => {
                let b = self.parse_boundary()?;
                Ok(ASTNode::Boundary(b))
            }
            TokenType::AffixedIdent { ref name, polarity, position } => {
                let id_name = name.clone();
                let line = tok.line;
                let col = tok.col;
                self.advance();
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
                self.advance();
                Ok(ASTNode::BoundaryCast(BoundaryCastNode {
                    boundary,
                    polarity,
                    line,
                    col,
                }))
            }
            TokenType::Aside(ref intent) => {
                self.advance();
                Ok(ASTNode::Aside(AsideNode {
                    intent: intent.clone(),
                    line: tok.line,
                    col: tok.col,
                }))
            }
            TokenType::Ternary(state) => {
                self.advance();
                Ok(ASTNode::Ternary(state))
            }
            TokenType::Number(val) => {
                self.advance();
                Ok(ASTNode::Number(val))
            }
            TokenType::StringLit(ref s) => {
                self.advance();
                Ok(ASTNode::StringLit(s.clone()))
            }
            TokenType::Infer => {
                self.advance();
                Ok(ASTNode::Identifier("?".into()))
            }
            TokenType::Pronoun(ref p) => {
                let name = p.as_str().to_string();
                self.advance();
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
                let next_tok = self.peek_at(1).token_type.clone();

                // 1. Entity Syntax: Name [ Definition ] ( Aside ) { Context }
                if matches!(next_tok, TokenType::LBracket | TokenType::LAngle | TokenType::LBrace) {
                    let id_name = id_str;
                    let line = tok.line;
                    let col = tok.col;
                    self.advance(); // consume identifier

                    let def_node = self.parse_boundary()?;
                    let mut aside_node = None;
                    let mut context_node = None;

                    // Check for optional Aside: ( ... )
                    let maybe_aside = if let TokenType::Aside(ref text) = self.peek().token_type {
                        Some(text.clone())
                    } else {
                        None
                    };
                    if let Some(text) = maybe_aside {
                        let a_tok = self.advance();
                        aside_node = Some(AsideNode {
                            intent: text,
                            line: a_tok.line,
                            col: a_tok.col,
                        });
                    }

                    // Check for optional Context boundary
                    if matches!(self.peek().token_type, TokenType::LBrace | TokenType::LBracket) {
                        context_node = Some(self.parse_boundary()?);
                    }

                    if def_node.boundary_type.is_immutable() {
                        self.debugger.register_immutable(&id_name);
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
                    if matches!(self.peek_at(2).token_type, TokenType::LBrace | TokenType::LBracket | TokenType::LAngle) {
                        let id_name = id_str;
                        let line = tok.line;
                        let col = tok.col;
                        let aside_text = text.clone();
                        self.advance(); // consume identifier
                        let a_tok = self.advance(); // consume aside
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

                self.advance();
                Ok(ASTNode::Identifier(id_str))
            }
            _ => {
                let msg = format!("Unexpected token '{:?}' in expression at L{}:C{}", tok.token_type, tok.line, tok.col);
                self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "PARSER", &msg);
                Err(format!("FAILPOINT: {msg}"))
            }
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Parsing Invariant Verification, Boundary Integrity Proofs & Test Oracles )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    #[test]
    fn test_parser_leading_equals_module_declaration() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let src = "= [Module]: < Lexicon { Types } > & [ Noun { Character } ] { [ Version ]: { \"1.0.0\" } }";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "Module declaration starting with '=' must parse cleanly: {:?}", prog.err());
    }

    #[test]
    fn test_parser_ranked_asymmetric_boundaries() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let src = "{ Input ] --> [ Output }";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "Asymmetric boundaries {{ ... ] and [ ... }} must parse cleanly: {:?}", prog.err());
    }

    #[test]
    fn test_parser_adversative_caveat_interlock() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let src = "Action [ +1 ] ! [ Guard = +1 ]";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "Adversative caveat ! must parse cleanly: {:?}", prog.err());
    }

    #[test]
    fn test_parser_character_omni_source_reel_full_parse() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        assert!(path.exists(), "character.omni must exist at resolved track path");
        let src = std::fs::read_to_string(&path).expect("character.omni must be readable");
        let mut lexer = crate::parser::lexer::Lexer::new(&src, logger);
        let tokens = lexer.tokenize().expect("character.omni lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "character.omni must parse cleanly into AST: {:?}", prog.err());
        let program_node = prog.unwrap();
        assert!(!program_node.statements.is_empty(), "character.omni AST must contain parsed statements");
    }

    #[test]
    fn test_parser_svo_verbs_and_composed_locators() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let src = "[ ROLL 4 ] : [ Project ] : [ Character` } --> PROJECT [ Target ] ::@ 0x01";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "SVO verb PROJECT and composed locator ::@ 0x01 must parse cleanly: {:?}", prog.err());
    }
}
