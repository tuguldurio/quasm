pub mod ast;
mod expr;
mod stmt;
mod types;

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
        self.tokens.get(self.pos).map(|t| &t.kind).unwrap_or(&TokenKind::Eof) == &kind
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

    fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        match self.peek() {
            TokenKind::Identifier(value) => {
                self.advance();
                Ok(Identifier { value })
            }
            other => Err(self.err(format!("expected identifier, got {:?}", other)))
        }
    }

    // Parses a {...} body of newline separated items
    fn parse_braced_list<T, F>(&mut self, label: &str, mut parse_item: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<T, ParseError>,
    {
        self.consume(TokenKind::LBrace)?;
        self.skip_newlines();
        let mut items = Vec::new();

        while self.peek_until(TokenKind::RBrace) {
            items.push(parse_item(self)?);
            self.expect_newline(label)?;
        }

        self.consume(TokenKind::RBrace)?;
        Ok(items)
    }
}
