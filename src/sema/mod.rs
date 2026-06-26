pub mod ty;
pub mod tast;
pub mod symbols;

use ty::Ty;
use crate::parser::ast;
use crate::common::ast::Literal;
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

    fn resolve_params_ty(&self, func: &ast::FuncStmt) -> Result<Vec<Ty>, SemaError> {
        let mut params_ty = Vec::new();
        for param in &func.params {
            let param_ty = match &param.ty {
                Some(ty) => self.resolve_ty(ty)?,
                None => Ty::Infer
            };
            params_ty.push(param_ty);
        }
        Ok(params_ty)
    }

    fn check_program(&mut self, ast: ast::Program) -> Result<tast::Program, SemaError> {
        // initial pass to register functions
        for stmt in &ast.stmts {
            match stmt {
                ast::Stmt::Func(func) => {
                    let name = func.name.value.clone();
                    let params_ty = self.resolve_params_ty(&func)?;
                    let ret = match &func.ret {
                        Some(ret) => self.resolve_ty(ret)?,
                        None => Ty::Unit
                    };
                    self.sym_table.define_func(&name, params_ty, ret);
                }
                ast::Stmt::Let(s) => {
                    return Err(self.err("not implemented yet", s.name.span));
                }
                ast::Stmt::Type(s) => {
                    return Err(self.err("not implemented yet", s.name.span));
                }
                ast::Stmt::Struct(s) => {
                    return Err(self.err("not implemented yet", s.name.span));
                }
                ast::Stmt::Expr(e) => {
                    return Err(self.err("top level should not contain expression", e.span));
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
            ast::Stmt::Expr(expr) => Ok(tast::Stmt::Expr(self.check_expr(expr)?))
        }
    }

    fn check_func(&mut self, func: ast::FuncStmt) -> Result<tast::FuncStmt, SemaError> {
        // lookup symtable
        let name = func.name.value;
        let first_param_ty = match func.params.first() {
            Some(param) => Some(self.resolve_ty(param.ty.as_ref().unwrap())?),
            None => None
        };

        let Some(func_symbol) = self.sym_table.lookup_func(&name, first_param_ty) else {
            return Err(self.err(format!("function `{}` is not declared", name), func.name.span));
        };
        let id = func_symbol.id;
        let params_ty = func_symbol.params_ty.clone();
        let ret_ty = func_symbol.ret_ty.clone();

        // enter func and build params
        self.sym_table.enter_func();
        let mut params = Vec::new();
        for (param, ty) in func.params.iter().zip(params_ty) {
            let id = self.sym_table.define_var(&param.name.value, ty.clone());
            params.push(tast::Param { id, ty });
        }

        // build body
        let body = self.check_block(func.body)?;

        if body.ty != ret_ty {
            return Err(self.err(
                format!(
                    "type mismatch for function `{}` return type: expected `{:?}`, got `{:?}`",
                    name, ret_ty, body.ty
                ),
                body.span
            ));
        }

        Ok(tast::FuncStmt { id, params, ret_ty, body })
    }

    fn check_let(&mut self, let_stmt: ast::LetStmt) -> Result<tast::Stmt, SemaError> {
        let value = self.check_expr(let_stmt.value)?;

        let annot_ty = match &let_stmt.annot_ty {
            Some(annot) => {
                let annot_ty = self.resolve_ty(annot)?;
                if annot_ty != value.ty {
                    return Err(self.err(
                        format!(
                            "type mismatch for {}: expected `{:?}`, got `{:?}`",
                            let_stmt.name.value, annot_ty, value.ty
                        ),
                        let_stmt.name.span
                    ));
                }
                annot_ty
            }
            None => value.ty.clone()
        };

        let id = self.sym_table.define_var(&let_stmt.name.value, annot_ty.clone());

        // a let statement itself evaluates to unit
        Ok(tast::Stmt::Let(tast::LetStmt { id, value, annot_ty, ty: Ty::Unit }))
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
            _ => Ty::Unit
        };

        Ok(tast::Block { stmts, ty, span })
    }

    fn check_expr(&mut self, expr: ast::Expr) -> Result<tast::Expr, SemaError> {
        match expr.kind {
            ast::ExprKind::Literal(lit) => {
                let ty = match lit {
                    Literal::Int(_) => Ty::Int,
                    Literal::Float(_) => Ty::Float,
                    Literal::Bool(_) => Ty::Bool
                };
                Ok(tast::Expr { kind: tast::ExprKind::Literal(lit), ty })
            }
            ast::ExprKind::Block(block) => {
                let block = self.check_block(block)?;
                let ty = block.ty.clone();
                Ok(tast::Expr { kind: tast::ExprKind::Block(block), ty })
            }
            ast::ExprKind::Identifier(identifier) => {
                let Some((id, ty)) = self.sym_table.lookup_var(&identifier.value) else {
                    return Err(self.err(
                        format!("cannot find `{}` in this scope", identifier.value),
                        identifier.span
                    ));
                };
                Ok(tast::Expr { kind: tast::ExprKind::Var { id }, ty })
            }
            ast::ExprKind::BinaryOp { op, left, right } => {
                let span = expr.span;
                let left = self.check_expr(*left)?;
                let right = self.check_expr(*right)?;

                let Some(ty) = ty::bin_op_ty(op, &left.ty, &right.ty) else {
                    return Err(self.err(
                        format!("invalid binary operation: `{:?}` {} `{:?}`", left.ty, op, right.ty),
                        span
                    ));
                };

                Ok(tast::Expr {
                    kind: tast::ExprKind::BinaryOp { op, left: Box::new(left), right: Box::new(right) },
                    ty
                })
            }
            // other expression kinds aren't checked yet
            _ => Ok(tast::Expr { kind: tast::ExprKind::Error, ty: Ty::Unit })
        }
    }
}