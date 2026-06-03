use clap::Parser as ClapParser;
use quasm::{lexer, parser::Parser};
use std::{fs, path::PathBuf};

#[derive(ClapParser)]
struct Args {
    file: PathBuf,
    #[arg(short = 'd', long = "debug")]
    debug: bool
}

fn main() {
    let args = Args::parse();
    let src = fs::read_to_string(&args.file).unwrap_or_else(|e| {
        eprintln!("error reading '{}': {}", args.file.display(), e);
        std::process::exit(1);
    });

    let (tokens, lexer_errors) = lexer::lex(&src);
    for error in &lexer_errors {
        eprintln!("lex error: {} {:?}", error.message, error.span);
    }

    if args.debug {
        fs::create_dir_all("build").unwrap_or_else(|e| {
            eprintln!("error creating build/: {}", e);
            std::process::exit(1);
        });
        let tokens_out: String = tokens
            .iter()
            .map(|t| format!("{:?} {:?}", t.kind, t.span))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write("build/tokens.txt", tokens_out).unwrap_or_else(|e| {
            eprintln!("error writing build/tokens.txt: {}", e);
        });
    }

    let mut parser = Parser::new(tokens);
    match parser.parse_program() {
        Ok(program) => {
            if args.debug {
                fs::write("build/ast.txt", format!("{:#?}", program)).unwrap_or_else(|e| {
                    eprintln!("error writing build/ast.txt: {}", e);
                });
            }
        }
        Err(e) => eprintln!("parse error: {} {:?}", e.message, e.span),
    }
}
