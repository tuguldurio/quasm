// prototype IR
// todo: decide IR design
#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub u64);

#[derive(Debug)]
pub struct Module {
    pub heap_types: Vec<HeapType>,
    // pub types: Vec<IrType>,
    pub funcs: Vec<Func>,
    pub entry: Option<FuncId>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrTy {
    I32,
    I64,
    F64,
    Unit // erased by codegen (no wasm value)
}

#[derive(Debug)]
pub enum HeapType {
    Struct { fields: Vec<IrTy> },
    EnumHeader { variants: Vec<TypeId> }
}

#[derive(Debug)]
pub struct Func {
    pub id: FuncId,
    pub params: Vec<Param>,
    pub ret_ty: IrTy,
    pub body: Expr
}

#[derive(Debug)]
pub struct Param {
    pub id: LocalId,
    pub ty: IrTy
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: IrTy
}

#[derive(Debug)]
pub enum ExprKind {
    ConstInt(i64),
    ConstFloat(f64),
    ConstBool(bool),
    Unit,

    LocalGet(LocalId),
    LocalSet { id: LocalId, value: Box<Expr> },
    Let { id: LocalId, value: Box<Expr> },

    Block(Vec<Expr>),

    Call { func: FuncId, args: Vec<Expr> },
    FuncRef(FuncId),

    StructNew { ty: TypeId, fields: Vec<Expr> },
    StructGet {
        obj: Box<Expr>,
        ty: TypeId,
        field: u64
    }
}
