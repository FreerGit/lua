use crate::ast::*;
use crate::lex::{Lex, Token};

#[derive(Debug)]
pub enum Error {
    SyntaxError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Parser<'a> {
    lexer: Lex<'a>,
    current: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lex<'a>) -> Self {
        let current = lexer.next();
        Self { lexer, current }
    }

    /// Advance to the next token
    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn expect(&mut self, expected: Token<'a>) -> Result<()> {
        if std::mem::discriminant(&self.current) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(Error::SyntaxError(format!(
                "Expected {:?}, got {:?} at line {}:{}",
                expected,
                self.current,
                self.lexer.line_number(),
                self.lexer.line_position()
            )))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<StmtNode>> {
        let mut stmts = Vec::new();
        while self.current != Token::Eof {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<StmtNode> {
        let start_span = self.lexer.line_number();

        let stmt = match self.current {
            // Token::If => self.if_statement(),
            // Token::While => self.while_statement(),
            // Token::Repeat => self.repeat_statement(),
            // Token::For => self.for_statement(),
            // Token::Function => self.function_statement(),
            // Token::Local => self.local_statement(),
            // Token::Return => self.return_statement(),
            Token::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            _ => {
                let expr = self.expression()?;
                match &expr.expr {
                    Expr::FuncCall(_, _) => Ok(Stmt::FuncCall(expr)),
                    Expr::MethodCall(_, _, _) => Ok(Stmt::MethodCall(expr)),
                    _ => Ok(Stmt::Assign(vec![expr], vec![])), // fallback
                }
            }
        }?;

        let end_span = self.lexer.line_number();
        Ok(StmtNode::new(stmt, (start_span, end_span)))
    }

    fn expression(&mut self) -> Result<ExprNode> {
        let start_span = self.lexer.line_number();

        let expr = match self.current {
            Token::False => Expr::Bool(false),
            Token::Nil => Expr::Nil,
            Token::Integer(n) => Expr::Integer(n),
            Token::Float(f) => Expr::Float(f),
            Token::Name(s) => {
                let name = s.to_string();
                self.advance();

                if self.current == Token::ParL {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current != Token::ParR {
                        loop {
                            args.push(self.expression()?);
                            if self.current == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::ParR)?;
                    Expr::FuncCall(
                        Box::new(ExprNode::new(
                            Expr::Ident(name),
                            (start_span, self.lexer.line_number()),
                        )),
                        args,
                    )
                } else {
                    Expr::Ident(name)
                }
            }
            Token::ParL => {
                self.advance();
                let inner = self.expression()?;
                self.expect(Token::ParR)?;
                return Ok(inner);
            }
            _ => {
                return Err(Error::SyntaxError(format!(
                    "Unexpected token {:?} at line {}:{}",
                    self.current,
                    self.lexer.line_number(),
                    self.lexer.line_position()
                )));
            }
        };

        self.advance();
        let end_span = self.lexer.line_number();
        Ok(ExprNode::new(expr, (start_span, end_span)))
    }

    // TODO: Implement full statement and expression parsing (if, while, repeat, for, functions, etc.)
}
