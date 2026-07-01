use crate::common::ast::{Literal, BinOpKind, UnaryOpKind};
use crate::sema::ty::Ty;
use crate::common::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructFieldId(pub u64);

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>
}

#[derive(Debug)]
pub enum Stmt {
    Func(Func),
    Struct(Struct),
    Let(Let),
    Var(Var),
    Expr(Expr)
}

#[derive(Debug)]
pub struct Func {
    pub id: FuncId,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body: Block
}

#[derive(Debug)]
pub struct Param {
    pub id: VarId,
    pub ty: Ty
}

#[derive(Debug)]
pub struct Struct {
    pub id: StructId,
    pub fields: Vec<StructField>,
    pub ty: Ty
}

#[derive(Debug)]
pub struct StructField {
    pub id: StructFieldId,
    pub ty: Ty
}

#[derive(Debug)]
pub struct StructLit {
    pub id: StructId,
    pub fields: Vec<Expr>,
    pub ty: Ty
}

#[derive(Debug)]
pub struct Let {
    pub id: VarId,
    pub value: Expr,
    pub annot_ty: Ty,
    pub ty: Ty
}

#[derive(Debug)]
pub struct Var {
    pub id: VarId,
    pub value: Expr,
    pub annot_ty: Ty,
    pub ty: Ty
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Ty
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
    Var { id: VarId },
    Func { id: FuncId },
    Block(Block),
    BinaryOp(BinaryOp),
    UnaryOp {
        op: UnaryOpKind,
        operand: Box<Expr>
    },
    Call(Call),
    StructLit(StructLit),
    // stands in for an expression that failed to check or isn't supported yet
    Error
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub ty: Ty,
    pub span: Span
}

#[derive(Debug)]
pub struct BinaryOp {
    pub op: BinOpKind,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub ty: Ty
}

#[derive(Debug)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub ty: Ty
}