#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement {
    FuncStatement(FuncDecl),
    ExpressionStatement(Expression),
}

#[derive(Debug)]
pub struct FuncDecl {
    pub name: Identifier,
    pub body: Block,
}

#[derive(Debug)]
pub struct Param {
    pub name: String
}

// Statements in a block are newline-separated; the last Stmt::Expr is the implicit return value.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    Int(Int),
    Bool(Bool),
    Identifier(Identifier),
    BinaryOp {
        op: BinOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct Int {
    pub value: i64,
}

#[derive(Debug)]
pub struct Bool {
    pub value: bool,
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
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
    And, Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg, // -x
    Not, // !x
}
