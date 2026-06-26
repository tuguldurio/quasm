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
            ast::Stmt::Func(func) => Ok(tast::Stmt::Func(self.check_func(func)?)),
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

    fn check_func(&mut self, func: ast::FuncStmt) -> Result<tast::FuncStmt, SemaError> {
        let (name, first_param_ty) = self.func_key(&func)?;
        let id = self.sym_table.lookup_func(name, first_param_ty);

        let mut params = Vec::new();
        for param in func.params {
            let ty = self.resolve_ty(&param.ty.unwrap())?;
            params.push(tast::Param { name: param.name, id: params.len() as u64, ty });
        }

        let ret_ty = match &func.ret {
            Some(r) => self.resolve_ty(r)?,
            None => Ty::Unit,
        };

        let body = self.check_block(func.body)?;

        if body.ty != ret_ty {
            return Err(self.err(
                format!(
                    "function `{}` returns `{:?}` but its body has type `{:?}`",
                    func.name.value, ret_ty, body.ty
                ),
                body.span,
            ));
        }

        Ok(tast::FuncStmt { id, name: func.name, params, body })
    }

    fn check_let(&mut self, let_stmt: ast::LetStmt) -> Result<tast::Stmt, SemaError> {
        Err(self.err("let not implemented yet", let_stmt.name.span))
    }

    fn check_block(&mut self, block: ast::Block) -> Result<tast::Block, SemaError> {
        let span = block.span;

        let mut stmts = Vec::new();
        for stmt in block.stmts {
            stmts.push(self.check_statement(stmt)?);
        }

        // a block evaluates to its trailing expression, otherwise to unit
        let ty = match stmts.last() {
            Some(tast::Stmt::Expr(expr)) => expr.ty.clone(),
            _ => Ty::Unit,
        };

        Ok(tast::Block { stmts, ty, span })
    }

    fn check_expr(&mut self, expr: ast::Expr) -> Result<tast::Expr, SemaError> {
        match expr.kind {
            ast::ExprKind::Literal(lit) => {
                let ty = match lit {
                    Literal::Int(_) => Ty::Int,
                    Literal::Float(_) => Ty::Float,
                    Literal::Bool(_) => Ty::Bool,
                };
                Ok(tast::Expr { kind: tast::ExprKind::Literal(lit), ty })
            }
            ast::ExprKind::Block(block) => {
                let block = self.check_block(block)?;
                let ty = block.ty.clone();
                Ok(tast::Expr { kind: tast::ExprKind::Block(block), ty })
            }
            // other expression kinds aren't checked yet
            _ => Ok(tast::Expr { kind: tast::ExprKind::Error, ty: Ty::Unit }),
        }
    }
}