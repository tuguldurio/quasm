pub mod token;

pub use token::TokenKind;

use logos::Logos;

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub span: std::ops::Range<usize>
}

pub struct LexError {
    pub span: std::ops::Range<usize>,
    pub message: String
}

pub fn lex(src: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    let mut lexer = TokenKind::lexer(src);
    
    while let Some(tok) = lexer.next() {
        let span = lexer.span();
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

    (tokens, errors)
}