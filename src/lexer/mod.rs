pub mod token;

pub use token::TokenKind;

use crate::common::span::{LineMap, Span};
use logos::Logos;

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub span: Span
}

pub struct LexError {
    pub span: Span,
    pub message: String
}

pub fn lex(src: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let line_map = LineMap::new(src);

    let mut lexer = TokenKind::lexer(src);

    while let Some(tok) = lexer.next() {
        let span = line_map.span(lexer.span());
        let literal = lexer.slice().to_string();

        match tok {
            Ok(kind) => tokens.push(Token {
                kind,
                literal,
                span
            }),
            Err(_) => errors.push(LexError {
                span,
                message: format!("`{}`", literal)
            })
        }
    }

    let end = src.len();
    tokens.push(Token { kind: TokenKind::Eof, literal: String::new(), span: line_map.span(end..end) });

    (tokens, errors)
}
