pub mod ast;
mod expr;

use crate::lexer::{Token, TokenKind};
use ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Option<std::ops::Range<usize>>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> TokenKind {
        self.tokens.get(self.pos).map(|t| t.kind.clone()).unwrap_or(TokenKind::Eof)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), TokenKind::Newline) {
            self.advance();
        }
    }

    fn consume(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        let k = self.peek();
        if k == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.err(format!("expected {:?}, got {:?}", expected, k)))
        }
    }

    fn current_span(&self) -> Option<std::ops::Range<usize>> {
        self.tokens.get(self.pos).map(|t| t.span.clone())
    }

    fn err(&self, msg: impl Into<String>) -> ParseError {
        ParseError { message: msg.into(), span: self.current_span() }
    }
}

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while self.peek() != TokenKind::Eof {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }
        
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            TokenKind::Func => Ok(Statement::Func(self.parse_func_decl()?)),
            TokenKind::Let => Ok(Statement::Let(self.parse_let_statement()?)),
            _ => Ok(Statement::Expression(self.parse_expr()?)),
        }
    }

    fn parse_func_decl(&mut self) -> Result<FuncStmt, ParseError> {
        self.consume(TokenKind::Func)?;
        let name = self.parse_identifier()?;
        let params = self.parse_params()?;
        let ret = self.parse_type_annotation()?;
        let body = self.parse_block()?;
        Ok(FuncStmt { name, params, ret, body })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        self.consume(TokenKind::LParen)?;
        let mut params = Vec::new();

        while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
            let name = self.parse_identifier()?;
            let ty = self.parse_type_annotation()?
                .ok_or_else(|| self.err("expected type annotation for parameter"))?;
            params.push(Param { name, ty });

            if self.peek() == TokenKind::Comma {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(TokenKind::RParen)?;
        Ok(params)
    }

    fn parse_let_statement(&mut self) -> Result<LetStmt, ParseError> {
        self.consume(TokenKind::Let)?;
        let name = self.parse_identifier()?;
        let ty = self.parse_type_annotation()?;
        self.consume(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        Ok(LetStmt { name, ty, value })
    }

    fn parse_type_annotation(&mut self) -> Result<Option<Identifier>, ParseError> {
        if self.peek() != TokenKind::Colon {
            return Ok(None);
        }
        self.advance();
        Ok(Some(self.parse_identifier()?))
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.consume(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        self.skip_newlines();

        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_statement()?);

            match self.peek() {
                TokenKind::Newline | TokenKind::RBrace | TokenKind::Eof => {}
                other => {
                    return Err(self.err(format!("expected newline after statement, got {:?}", other)))
                }
            }
            
            self.skip_newlines();
        }

        self.consume(TokenKind::RBrace)?;
        Ok(Block { stmts })
    }

}