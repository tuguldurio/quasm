use std::collections::HashMap;
use crate::sema::ty::Ty;

#[derive(PartialEq, Eq, Hash)]
struct FuncKey {
    name: String,
    first_param_ty: Option<Ty> // None for func with no param
}

pub struct FuncSymbol {
    pub id: u64,
    pub params_ty: Vec<Ty>,
    pub ret_ty: Ty
}

pub struct VarSymbol {
    pub id: u64,
    pub ty: Ty
}

pub struct SymbolTable {
    // symbols: HashMap<>
    funcs: HashMap<FuncKey, FuncSymbol>,
    vars: HashMap<String, VarSymbol>,
    next_var_id: u64
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            vars: HashMap::new(),
            next_var_id: 0
        }
    }

    pub fn define_func(&mut self, name: &str, params_ty: Vec<Ty>, ret_ty: Ty) {
        let key = FuncKey {
            name: name.to_string(),
            first_param_ty: params_ty.first().cloned()
        };
        let id = self.funcs.len() as u64;

        self.funcs.insert(
            key,
            FuncSymbol { id, params_ty, ret_ty }
        );
    }

    pub fn lookup_func(&self, name: &str, first_param_ty: Option<Ty>) -> Option<&FuncSymbol> {
        self.funcs.get(&FuncKey { name: name.to_string(), first_param_ty })
    }

    pub fn enter_func(&mut self) {
        self.vars.clear();
        self.next_var_id = 0;
    }

    pub fn define_var(&mut self, name: String, ty: Ty) -> u64 {
        let id = self.next_var_id;
        self.next_var_id += 1;
        self.vars.insert(name, VarSymbol { id, ty });
        id
    }

    pub fn lookup_var(&self, name: &str) -> Option<(u64, Ty)> {
        self.vars.get(name).map(|v| (v.id, v.ty.clone()))
    }
}