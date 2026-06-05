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
    pub ty: Option<Identifier>,
    pub value: Expr
}

#[derive(Debug)]
pub struct FuncStmt {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub ret: Option<Identifier>,
    pub body: Block
}

#[derive(Debug)]
pub struct Param {
    pub name: Identifier,
    pub ty: Identifier
}

// Statements in a block are newline-separated; the last Stmt::Expr is the implicit return value.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>
}

#[derive(Debug)]
pub enum Expr {
    Int(IntLit),
    Bool(BoolLit),
    Identifier(Identifier),
    Call {
        callee: Identifier,
        args: Vec<Expr>
    },
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>
    }
}

#[derive(Debug)]
pub struct IntLit {
    pub value: i64
}

#[derive(Debug)]
pub struct BoolLit {
    pub value: bool
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
    pub ty_fields: Vec<Identifier>
}

#[derive(Debug)]
pub struct StructStmt {
    pub name: Identifier,
    pub fields: Vec<StructField>
}

#[derive(Debug)]
pub struct StructField {
    pub name: Identifier,
    pub ty: Identifier
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
