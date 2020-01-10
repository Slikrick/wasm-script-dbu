extern crate wasm_bindgen;
extern crate indexmap;

use indexmap::{IndexMap, map::Entry};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn dedupe(msg: &str) -> String {
    let mut map = IndexMap::new();
    let names = msg.replace(|c: char|c.is_digit(10), "").replace("\n", "  ").replace(" Reserver ", "  ");
    let iter = names.split("  ")
                    .map(str::trim)
                    .filter(|s| !s.is_empty());
    for name in iter {
        match map.entry(name) {
           Entry::Occupied(mut occ) => { *occ.get_mut() += 1 }
           Entry::Vacant(vac) => { vac.insert(1i32); }
        }
    }
    let mut str_buf = String::new();
    for (name, number) in &map {
       str_buf.push_str(&format!("{} {}\n", number, name));
    }
    str_buf
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}