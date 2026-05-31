use quasm::lexer;
use std::fs;

fn main() {
    let src = fs::read_to_string("examples/simple.qsm").unwrap();

    let (tokens, errors) = lexer::lex(&src);
    for error in errors {
        eprintln!("lex error: {} {:?}", error.message, error.span);
    }
    for token in tokens {
        println!("{:?} {:?}", token.kind, token.span);
    }
}
