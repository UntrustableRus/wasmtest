#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn eval(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    eval("console.log('hello');");
}
