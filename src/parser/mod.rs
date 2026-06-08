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

    fn peek_is(&self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    fn peek_until(&self, kind: TokenKind) -> bool {
        !self.peek_is(TokenKind::Eof) && !self.peek_is(kind)
    }

    fn skip_newlines(&mut self) {
        while self.peek_is(TokenKind::Newline) {
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

    fn expect_newline(&mut self, context: &str) -> Result<(), ParseError> {
        match self.peek() {
            TokenKind::Newline | TokenKind::RBrace | TokenKind::Eof => {}
            other => return Err(self.err(format!("expected newline after {}, got {:?}", context, other)))
        }
        self.skip_newlines();
        Ok(())
    }

    // Parses comma list with singleline/multiline enforcement
    // Multiline mode is triggered when '(' is immediately followed by a newline
    fn parse_comma_list<T, F>(&mut self, open: TokenKind, close: TokenKind, label: &str, mut parse_item: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<T, ParseError>,
    {
        self.consume(open)?;
        let multiline = self.peek_is(TokenKind::Newline);
        self.skip_newlines();
        let mut items = Vec::new();

        while self.peek_until(close.clone()) {
            items.push(parse_item(self)?);

            if !self.peek_is(TokenKind::Comma) {
                if multiline && !self.peek_is(TokenKind::Newline) {
                    return Err(self.err(format!("closing {:?} must be on its own line in multiline style", close)));
                }
                break;
            }
            self.consume(TokenKind::Comma)?;
            if multiline {
                self.skip_newlines();
            } else if self.peek_is(TokenKind::Newline) {
                return Err(self.err(format!("{label}s must all be on the same line; use a newline after '(' for multiline style")));
            }
        }

        self.skip_newlines();
        if self.peek_is(TokenKind::Comma) {
            return Err(self.err(format!("comma must follow a {label} on the same line, not precede it on the next line")));
        }
        self.consume(close)?;
        Ok(items)
    }
}

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !self.peek_is(TokenKind::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            TokenKind::Func => Ok(Stmt::Func(self.parse_func_decl()?)),
            TokenKind::Let => Ok(Stmt::Let(self.parse_let_statement()?)),
            TokenKind::Type => Ok(Stmt::Type(self.parse_type_decl()?)),
            TokenKind::Struct => Ok(Stmt::Struct(self.parse_struct_decl()?)),
            _ => Ok(Stmt::Expr(self.parse_expr()?)),
        }
    }

    fn parse_func_decl(&mut self) -> Result<FuncStmt, ParseError> {
        self.consume(TokenKind::Func)?;
        let name = self.parse_identifier()?;
        let params = self.parse_func_params()?;
        let ret = if self.peek_is(TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(FuncStmt { name, params, ret, body })
    }

    fn parse_func_params(&mut self) -> Result<Vec<Param>, ParseError> {
        self.parse_comma_list(TokenKind::LParen, TokenKind::RParen, "parameter", |p| {
            let name = p.parse_identifier()?;
            let ty = p.parse_type_annotation()?
                .ok_or_else(|| p.err("expected type annotation for parameter"))?;
            Ok(Param { name, ty })
        })
    }

    fn parse_let_statement(&mut self) -> Result<LetStmt, ParseError> {
        self.consume(TokenKind::Let)?;
        let name = self.parse_identifier()?;
        let ty = self.parse_type_annotation()?;
        self.consume(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        Ok(LetStmt { name, ty, value })
    }

    fn parse_type_decl(&mut self) -> Result<TypeStmt, ParseError> {
        self.consume(TokenKind::Type)?;
        let name = self.parse_identifier()?;

        let ty_params = if self.peek_is(TokenKind::LParen) {
            self.parse_comma_list(TokenKind::LParen, TokenKind::RParen, "type parameter", |p| p.parse_identifier())?
        } else {
            Vec::new()
        };

        self.consume(TokenKind::LBrace)?;
        self.skip_newlines();

        let mut variants = Vec::new();
        while self.peek_until(TokenKind::RBrace) {
            variants.push(self.parse_type_variant()?);
            self.expect_newline("variant")?;
        }

        self.consume(TokenKind::RBrace)?;
        Ok(TypeStmt { name, ty_params, variants })
    }

    fn parse_type_variant(&mut self) -> Result<TypeVariant, ParseError> {
        let name = self.parse_identifier()?;

        let ty_fields = if self.peek_is(TokenKind::LParen) {
            self.parse_comma_list(TokenKind::LParen, TokenKind::RParen, "type field", |p| p.parse_type())?
        } else {
            Vec::new()
        };

        Ok(TypeVariant { name, ty_fields })
    }

    fn parse_struct_decl(&mut self) -> Result<StructStmt, ParseError> {
        self.consume(TokenKind::Struct)?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::LBrace)?;
        self.skip_newlines();

        let mut fields = Vec::new();
        while self.peek_until(TokenKind::RBrace) {
            fields.push(self.parse_struct_field()?);
            self.expect_newline("field")?;
        }

        self.consume(TokenKind::RBrace)?;
        Ok(StructStmt { name, fields })
    }

    fn parse_struct_field(&mut self) -> Result<StructField, ParseError> {
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        Ok(StructField { name, ty })
    }

    fn parse_type_annotation(&mut self) -> Result<Option<Type>, ParseError> {
        if !self.peek_is(TokenKind::Colon) {
            return Ok(None);
        }
        self.advance();
        Ok(Some(self.parse_type()?))
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.peek() {
            TokenKind::LBracket => {
                self.advance();
                let inner = self.parse_type()?;
                self.consume(TokenKind::RBracket)?;
                Ok(Type::Array(Box::new(inner)))
            }
            TokenKind::Identifier(_) => Ok(Type::Named(self.parse_identifier()?)),
            other => Err(self.err(format!("expected type, got {:?}", other)))
        }
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.consume(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        self.skip_newlines();

        while self.peek_until(TokenKind::RBrace) {
            stmts.push(self.parse_statement()?);
            self.expect_newline("statement")?;
        }

        self.consume(TokenKind::RBrace)?;
        Ok(Block { stmts })
    }
}
