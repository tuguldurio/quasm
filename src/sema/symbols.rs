use std::collections::HashMap;
use crate::sema::ty::Ty;

#[derive(PartialEq, Eq, Hash)]
struct FuncKey {
    name: String,
    first_param: Option<Ty>
}

pub struct SymbolTable {
    funcs: HashMap<FuncKey, usize>
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new()
        }
    }

    pub fn define_func(&mut self, name: String, first_param: Option<Ty>) {
        let key = FuncKey {
            name,
            first_param
        };
        let func_id = self.funcs.len();
        self.funcs.insert(key, func_id);
    }
}