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
    scopes: Vec<HashMap<String, VarSymbol>>,
    next_local_id: u64
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            scopes: Vec::new(),
            next_local_id: 0
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

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }
    
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn enter_func(&mut self) {
        self.enter_scope();
        self.next_local_id = 0;
    }

    pub fn exit_func(&mut self) {
        self.exit_scope();
    }

    pub fn define_var(&mut self, name: &str, ty: Ty) -> Result<u64, String> {
        let scope = self.scopes.last_mut()
            .expect("bug: idk why but for some reason define_var is called without any scope");

        if scope.contains_key(name) {
            return Err(format!("variable `{name}` is alredy defined"))
        }

        let id = self.next_local_id;
        self.next_local_id += 1;

        scope.insert(name.to_string(), VarSymbol {id, ty});
        Ok(id)
    }

    pub fn lookup_var(&self, name: &str) -> Option<&VarSymbol> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }
}