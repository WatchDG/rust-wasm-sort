extern crate wasm_bindgen;
extern crate js_sys;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
    let l = v.len();
    for i in 0..(l - 1) {
        for j in (i + 1)..l {
            if v[i] > v[j] {
                v.swap(i, j);
            }
        }
    }
}

#[wasm_bindgen]
pub fn bubble_sort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}