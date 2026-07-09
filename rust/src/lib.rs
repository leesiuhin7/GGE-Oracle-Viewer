mod data;
mod filter;
mod query;
mod sorting;
mod utils;
mod wasm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gge-oracle-viewer-wasm!");
}
