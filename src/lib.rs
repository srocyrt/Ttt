mod screen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn old(w: usize, h: usize) -> String {
    let map = screen::BitMap::white_noise(w, h);
    format!("{}", map)
}
