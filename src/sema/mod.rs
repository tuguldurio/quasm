pub mod ty;
pub mod tast;
pub mod scope;

use crate::common::span::Span;
use crate::parser::ast;
use scope::Scope;

pub struct Sema {
    scopes: Vec<Scope>
}

#[derive(Debug)]
pub struct SemaError {
    pub message: String,
    pub span: Span
}

pub fn check(ast: ast::Program) -> Result<tast::Program, SemaError> {
    Sema::new().visit_program(ast)
}

impl Sema {
    fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    fn visit_program(&mut self, ast: ast::Program) -> Result<tast::Program, SemaError> {
        let _ = ast;
        Ok(tast::Program { statements: Vec::new() })
    }
}