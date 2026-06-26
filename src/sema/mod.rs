pub mod ty;
pub mod tast;
pub mod symbols;

use ty::Ty;
use crate::parser::ast;
use crate::common::span::Span;
use symbols::SymbolTable;

pub struct Sema {
    sym_table: SymbolTable
}

#[derive(Debug)]
pub struct SemaError {
    pub message: String,
    pub span: Span
}

pub fn check(ast: ast::Program) -> Result<tast::Program, SemaError> {
    Sema::new().check_program(ast)
}

impl Sema {
    fn new() -> Self {
        Self {
            sym_table: SymbolTable::new()
         }
    }

    fn err(&self, message: impl Into<String>, span: Span) -> SemaError {
        SemaError { message: message.into(), span }
    }

    fn resolve_ty(&self, ty: &ast::Ty) -> Result<Ty, SemaError> {
        match ty {
            ast::Ty::Named { name, args } => {
                if !args.is_empty() {
                    return Err(self.err("generic types are not supported yet", name.span));
                }

                match name.value.as_str() {
                    "Int"   if args.is_empty() => Ok(Ty::Int),
                    "Float" if args.is_empty() => Ok(Ty::Float),
                    "Bool"  if args.is_empty() => Ok(Ty::Bool),
                    _ => Err(self.err(format!("unknown type `{}`", name.value), name.span))
                }
            }
            ast::Ty::Array(inner) => {
                Ok(Ty::Array(Box::new(self.resolve_ty(inner)?)))
            }
            ast::Ty::Func { params, ret } => {
                let params = params.iter().map(|p| self.resolve_ty(p)).collect::<Result<_,_>>()?;
                let ret = match ret {
                    Some(r) => self.resolve_ty(r)?,
                    None => Ty::Unit
                };
                Ok(Ty::Func { params, ret: Box::new(ret) })
            }
        }
    }

    fn func_key(&self, func: &ast::FuncStmt) -> Result<(String, Option<Ty>), SemaError> {
        let name = func.name.value.clone();
        let first_param_ty = match func.params.first() {
            Some(p) => Some(self.resolve_ty(p.ty.as_ref().unwrap())?),
            None => None,
        };
        Ok((name, first_param_ty))
    }

    fn check_program(&mut self, ast: ast::Program) -> Result<tast::Program, SemaError> {
        // initial pass to register functions
        for stmt in &ast.stmts {
            match stmt {
                ast::Stmt::Func(func) => {
                    let (name, first_param_ty) = self.func_key(func)?;
                    self.sym_table.define_func(name, first_param_ty);
                }
                ast::Stmt::Let(s) => {
                    return Err(self.err("Not implemented yet", s.name.span));
                }
                ast::Stmt::Type(s) => {
                    return Err(self.err("Not implemented yet", s.name.span));
                }
                ast::Stmt::Struct(s) => {
                    return Err(self.err("Not implemented yet", s.name.span));
                }
                ast::Stmt::Expr(e) => {
                    return Err(self.err("Top level should not contain expression", e.span));
                }
            }
        }

        // second pass to build tast
        let mut stmts = Vec::new();

        for stmt in ast.stmts {
            stmts.push(self.check_statement(stmt)?);
        }

        Ok(tast::Program { stmts })
    }

    fn check_statement(&mut self, stmt: ast::Stmt) -> Result<tast::Stmt, SemaError> {
        match stmt {
            ast::Stmt::Func(func) => self.check_func(func),
            ast::Stmt::Let(let_stmt) => self.check_let(let_stmt),
            ast::Stmt::Type(type_stmt) => {
                Err(self.err("type statement not implemented yet", type_stmt.name.span))
            }
            ast::Stmt::Struct(struct_stmt) => {
                Err(self.err("struct statement not implemented yet", struct_stmt.name.span))
            }
            ast::Stmt::Expr(expr) => {
                Err(self.err("expression statement not implemented yet", expr.span))
            }
        }
    }

    fn check_func(&mut self, func: ast::FuncStmt) -> Result<tast::Stmt, SemaError> {
        let (name, first_param_ty) = self.func_key(&func)?;
        let id = self.sym_table.lookup_func(name, first_param_ty);

        let params = Vec::new();
        for param in func.params {
            self.resolve_ty(&param.ty.unwrap());
        }

        let body = tast::Block {
            stmts: Vec::new(),
            ty: Ty::Unit,
            span: func.body.span
        };

        Ok(tast::Stmt::Func(tast::FuncStmt { id, name: func.name, params, body }))
    }

    fn check_let(&mut self, let_stmt: ast::LetStmt) -> Result<tast::Stmt, SemaError> {
        Err(self.err("let not implemented yet", let_stmt.name.span))
    }
}