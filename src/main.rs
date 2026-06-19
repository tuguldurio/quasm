use clap::Parser as ClapParser;
use quasm::{lexer, parser::Parser};
use std::{fs, path::PathBuf};

#[derive(ClapParser)]
struct Args {
    file: PathBuf,
    #[arg(short = 'd', long = "debug")]
    debug: bool
}

fn write_debug(name: &str, contents: &str) {
    let result = fs::create_dir_all("build").and_then(|_| fs::write(format!("build/{name}"), contents));
    if let Err(e) = result {
        eprintln!("error writing build/{name}: {e}");
    }
}

fn main() {
    let args = Args::parse();
    let src = fs::read_to_string(&args.file).unwrap_or_else(|e| {
        eprintln!("error reading '{}': {}", args.file.display(), e);
        std::process::exit(1);
    });

    let tokens = match lexer::lex(&src) {
        Ok(tokens) => tokens,
        Err(lexer_errors) => {
            for error in &lexer_errors {
                eprintln!("lex error: {} at {}", error.message, error.span);
            }
            std::process::exit(1);
        }
    };

    if args.debug {
        let tokens_out: String = tokens
            .iter()
            .map(|t| format!("{:?} {}", t.kind, t.span))
            .collect::<Vec<_>>()
            .join("\n");
        write_debug("tokens.txt", &tokens_out);
    }

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("parse error: {} at {}", e.message, e.span);
            std::process::exit(1);
        }
    };

    if args.debug {
        write_debug("ast.txt", &format!("{:#?}", ast));
    }
}
