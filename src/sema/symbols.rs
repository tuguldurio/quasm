use std::collections::HashMap;
use crate::sema::ty::Ty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u64);

#[derive(PartialEq, Eq, Hash)]
struct FuncKey {
    name: String,
    first_param_ty: Option<Ty> // None for func with no param
}

pub struct FuncSymbol {
    pub id: FuncId,
    pub params_ty: Vec<Ty>,
    pub ret_ty: Ty
}

pub struct VarSymbol {
    pub id: VarId,
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

    pub fn define_func(&mut self, name: &str, params_ty: Vec<Ty>, ret_ty: Ty) -> Result<(), String> {
        let key = FuncKey {
            name: name.to_string(),
            first_param_ty: params_ty.first().cloned()
        };

        if self.funcs.contains_key(&key) {
            let signature = match &key.first_param_ty {
                Some(ty) => format!("`{name}({ty:?}...)`"),
                None => format!("`{name}`"),
            };
            return Err(format!("function {signature} is already defined"));
        }

        let id = FuncId(self.funcs.len() as u64);

        self.funcs.insert(key, FuncSymbol { id, params_ty, ret_ty });
        Ok(())
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

    pub fn define_var(&mut self, name: &str, ty: Ty) -> Result<VarId, String> {
        let scope = self.scopes.last_mut()
            .expect("bug: idk why but for some reason define_var is called without any scope");

        if scope.contains_key(name) {
            return Err(format!("variable `{name}` is already defined"))
        }

        let id = VarId(self.next_local_id);
        self.next_local_id += 1;

        scope.insert(name.to_string(), VarSymbol {id, ty});
        Ok(id)
    }

    pub fn lookup_var(&self, name: &str) -> Option<&VarSymbol> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }
}