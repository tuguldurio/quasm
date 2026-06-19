#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
    Float,
    Bool,
    Unit,
    Array(Box<Ty>),
    Func { params: Vec<Ty>, ret: Box<Ty> }
    // ...
}