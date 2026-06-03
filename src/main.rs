use quasm::{lexer, parser::Parser};
use std::fs;

fn main() {
    let src = fs::read_to_string("examples/simpler.qsm").unwrap();

    let (tokens, lexer_errors) = lexer::lex(&src);
    for error in &lexer_errors {
        eprintln!("lex error: {} {:?}", error.message, error.span);
    }
    // for token in &tokens {
    //     println!("{:?} {:?}", token.kind, token.span);
    // }

    let mut parser = Parser::new(tokens);
    match parser.parse_program() {
        Ok(program) => println!("{:#?}", program),
        Err(e) => eprintln!("parse error: {} {:?}", e.message, e.span),
    }
}
