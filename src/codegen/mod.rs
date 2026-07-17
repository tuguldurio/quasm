#![allow(unused)]
use std::collections::HashMap;

use wasm_encoder::{
    TypeSection,
    ValType
};

pub struct Codegen {
    types: TypeSection,
    func_sig_cache: HashMap<(Vec<ValType>, Vec<ValType>), u32>
}

impl Codegen {
    fn func_sig_index(&mut self, params: Vec<ValType>, results: Vec<ValType>) -> u32 {
        let key = (params.clone(), results.clone());
        *self.func_sig_cache.entry(key).or_insert_with(|| {
            let idx = self.types.len();
            self.types.ty().function(params, results);
            idx
        })
    }
}