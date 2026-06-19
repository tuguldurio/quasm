use crate::sema::ty::Ty;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr)
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Ty
}

#[derive(Debug)]
pub enum ExprKind {

}
