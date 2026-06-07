#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>
}

#[derive(Debug)]
pub enum Stmt {
    Func(FuncStmt),
    Let(LetStmt),
    Enum(EnumStmt),
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
    pub receiver: Option<Identifier>,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Block
}

#[derive(Debug)]
pub struct Param {
    pub name: Identifier,
    pub ty: Type
}

// Statements in a block are newline-separated; the last Stmt::Expr is the implicit return value.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Identifier(Identifier),
    Array(Vec<Expr>),
    Block(Block),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>
    },
    Call {
        callee: Box<Expr>,
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
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String
}

#[derive(Debug)]
pub struct EnumStmt {
    pub name: Identifier,
    pub ty_params: Vec<Identifier>,
    pub variants: Vec<EnumVariant>
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: Identifier,
    pub ty_fields: Vec<Type>
}

#[derive(Debug)]
pub struct StructStmt {
    pub name: Identifier,
    pub fields: Vec<StructField>
}

#[derive(Debug)]
pub struct StructField {
    pub name: Identifier,
    pub ty: Type
}

#[derive(Debug)]
pub enum Type {
    Named(Identifier),
    Array(Box<Type>)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    // arithmetic
    Add, Sub, Mul, Div,
    // comparison
    Lt, Gt, LtEq, GtEq,
    // equality
    EqEq, NotEq,
    // logical
    And, Or
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not
}
