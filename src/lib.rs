#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod evaluator;
pub mod game;
pub mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
