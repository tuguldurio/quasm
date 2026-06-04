#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement {
    Func(FuncStmt),
    Let(LetStmt),
    Expression(Expression)
}

#[derive(Debug)]
pub struct LetStmt {
    pub name: Identifier,
    pub ty: Option<Identifier>,
    pub value: Expression
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
    pub ty: Identifier,
}

// Statements in a block are newline-separated; the last Stmt::Expr is the implicit return value.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Statement>
}

#[derive(Debug)]
pub enum Expression {
    Int(IntLit),
    Bool(BoolLit),
    Identifier(Identifier),
    Call {
        callee: Identifier,
        args: Vec<Expression>,
    },
    BinaryOp {
        op: BinOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expression>,
    }
}

#[derive(Debug)]
pub struct IntLit {
    pub value: i64,
}

#[derive(Debug)]
pub struct BoolLit {
    pub value: bool
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String
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
