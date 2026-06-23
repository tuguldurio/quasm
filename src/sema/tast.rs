use crate::common::ast::{Literal, BinOpKind, UnaryOpKind};
use crate::sema::ty::Ty;
use crate::common::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuncId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalId(pub u32);

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>
}

#[derive(Debug)]
pub enum Stmt {
    Func(FuncStmt),
    Let(LetStmt),
    Expr(Expr)
}

#[derive(Debug)]
pub struct FuncStmt {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub body: Block
}

#[derive(Debug)]
pub struct LetStmt {
    pub local: LocalId,
    pub value: Expr
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub ty: Ty,
    pub span: Span
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Ty
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
    Local(LocalId),
    Func(FuncId),
    Block(Block),
    BinaryOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>
    },
    UnaryOp {
        op: UnaryOpKind,
        operand: Box<Expr>
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>
    },
    // stands in for an expression that failed to check or isn't supported yet
    Error
}
