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
    pub stmts: Vec<Stmt>
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
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

// A bare identifier pattern may be a binding or a unit constructor; sema decides.
#[derive(Debug)]
pub enum Pattern {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool)
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String
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
