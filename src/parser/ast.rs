use crate::common::span::Span;
use crate::common::ast::{Literal, BinOpKind, UnaryOpKind};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>
}

#[derive(Debug)]
pub enum Stmt {
    Func(FuncStmt),
    Let(LetStmt),
    Type(TypeStmt),
    Struct(StructStmt),
    Expr(Expr)
}

#[derive(Debug)]
pub struct LetStmt {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Expr
}

#[derive(Debug)]
pub struct FuncStmt {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Block
}

#[derive(Debug)]
pub struct Param {
    pub name: Identifier,
    pub ty: Option<Type>
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
    Identifier(Identifier),
    Array(Vec<Expr>),
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
    UfcsCall {
        base: Box<Expr>,
        callee: Identifier,
        args: Vec<Expr>
    },
    Index {
        base: Box<Expr>,
        index: Box<Expr>
    },
    FieldAccess {
        base: Box<Expr>,
        field: Identifier
    },
    If {
        condition: Box<Expr>,
        then_block: Block,
        else_branch: Option<Box<Expr>>
    },
    Match {
        subject: Box<Expr>,
        arms: Vec<MatchArm>
    },
    Closure {
        params: Vec<Param>,
        ret: Option<Type>,
        body: Box<Expr>
    }
}

#[derive(Debug)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr
}

#[derive(Debug)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span
}

// A bare identifier pattern may be a binding or a unit constructor; sema decides.
#[derive(Debug)]
pub enum PatternKind {
    Wildcard,
    Literal(Literal),
    Identifier(Identifier),
    Constructor {
        name: Identifier,
        args: Vec<Pattern>
    },
    Array(Vec<Pattern>),
    // `..` binds the remainder of an array, optionally to a name;
    // only appears as the last element of an Array pattern
    Rest(Option<Identifier>),
    Or(Vec<Pattern>)
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
    pub span: Span
}

#[derive(Debug)]
pub struct TypeStmt {
    pub name: Identifier,
    pub ty_params: Vec<Identifier>,
    pub variants: Vec<TypeVariant>
}

#[derive(Debug)]
pub struct TypeVariant {
    pub name: Identifier,
    pub ty_fields: Vec<Type>
}

#[derive(Debug)]
pub struct StructStmt {
    pub name: Identifier,
    pub ty_params: Vec<Identifier>,
    pub fields: Vec<StructField>
}

#[derive(Debug)]
pub struct StructField {
    pub name: Identifier,
    pub ty: Type
}

// Lowercase named types are type variables, uppercase are concrete types.
#[derive(Debug)]
pub enum Type {
    Named {
        name: Identifier,
        args: Vec<Type>
    },
    Array(Box<Type>),
    Func {
        params: Vec<Type>,
        ret: Option<Box<Type>>
    }
}