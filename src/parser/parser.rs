// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Parser Identity, Boundary Physics Enforcement & Structural Invariants )
// ==============================================================================
//! OmniCode Recursive Descent Parser
//!
//! Converts tokens into an AST while enforcing boundary physics,
//! first-class asides, and failpoint validation through OmniDebugger.

use crate::ast::nodes::*;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::{Token, TokenType};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Parser State Initialization & Token Stream Context )
// ==============================================================================
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub debugger: OmniDebugger,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, debugger: OmniDebugger) -> Self {
        Self {
            tokens,
            pos: 0,
            debugger,
        }
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            self.tokens.last().unwrap()
        }
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn check(&self, expected: &TokenType) -> bool {
        &self.peek().token_type == expected
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Recursive-Descent Grammars: Program, Statements, Boundaries, Expressions )
// ==============================================================================
impl Parser {
    pub fn parse_program(&mut self) -> Result<ProgramNode, String> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::Eof) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.debugger.assert_all_boundaries_closed()?;
        Ok(ProgramNode { statements, line: 1, col: 1 })
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        self.parse_expression()
    }

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

        while !self.check(&TokenType::Eof) && !matches!(
            self.peek().token_type,
            TokenType::RBracket | TokenType::RBrace | TokenType::RAngle
        ) {
            let el = self.parse_expression()?;
            elements.push(el);
        }

        if self.check(&TokenType::Eof) {
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

        // Check for attached Aside: ( ... )
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

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_caveat_or_connective()
    }

    // Caveat ! BUT and Connectives ::, |
    fn parse_caveat_or_connective(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_directional()?;

        while matches!(
            self.peek().token_type,
            TokenType::But | TokenType::And | TokenType::Or | TokenType::InRelationTo
        ) {
            let op_tok = self.advance();
            let connective = match op_tok.token_type {
                TokenType::But => Connective::But,
                TokenType::And => Connective::And,
                TokenType::Or => Connective::Or,
                TokenType::InRelationTo => Connective::InRelationTo,
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

    // Directional Vectors: -->, <--, ^--, v--
    fn parse_directional(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_relation()?;

        while matches!(
            self.peek().token_type,
            TokenType::ActUpon | TokenType::ActAgain | TokenType::BuildsTo | TokenType::BreaksDownFrom
        ) {
            let op_tok = self.advance();
            let vector = match op_tok.token_type {
                TokenType::ActUpon => DirectionalVector::ActUpon,
                TokenType::ActAgain => DirectionalVector::ActAgain,
                TokenType::BuildsTo => DirectionalVector::BuildsTo,
                TokenType::BreaksDownFrom => DirectionalVector::BreaksDownFrom,
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

    // Being Relations: =, !=, ~=
    fn parse_relation(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_arithmetic()?;

        while matches!(
            self.peek().token_type,
            TokenType::Is | TokenType::IsNot | TokenType::ToBeLike
        ) {
            let op_tok = self.advance();
            let relation = match op_tok.token_type {
                TokenType::Is => BeingRelation::Is,
                TokenType::IsNot => BeingRelation::IsNot,
                TokenType::ToBeLike => BeingRelation::ToBeLike,
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

    // Arithmetic Math Operations: plus, minus, times, div (Natural SVO Keywords)
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

    // Unary Not: -- and Unary Prefix Vectors: <--, -->, ^--, v--
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

    // Primaries: Boundaries, Literals, Identifiers, Entities, Asides
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let tok = self.peek().clone();
        match tok.token_type {
            TokenType::LBracket | TokenType::LBrace | TokenType::LAngle => {
                let b = self.parse_boundary()?;
                Ok(ASTNode::Boundary(b))
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
            TokenType::Identifier(ref name) => {
                let next_tok = if self.pos + 1 < self.tokens.len() {
                    &self.tokens[self.pos + 1].token_type
                } else {
                    &TokenType::Eof
                };

                if matches!(next_tok, TokenType::LBracket | TokenType::LAngle | TokenType::LBrace) {
                    let id_name = name.clone();
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

                self.advance();
                Ok(ASTNode::Identifier(name.clone()))
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
// ( Parsing Invariant Verification & Teardown )
// ==============================================================================
// Parser AST generation pipeline verified and closed.
