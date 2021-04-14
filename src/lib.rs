extern crate js_sys;
extern crate sort;
extern crate wasm_bindgen;
extern crate wee_alloc;

use sort::bubble_sort;
use sort::merge_sort;
use sort::selection_sort;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn bubble_sort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_int16array(a: &js_sys::Int16Array) -> Vec<i16> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_int8array(a: &js_sys::Int8Array) -> Vec<i8> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_uint32array(a: &js_sys::Uint32Array) -> Vec<u32> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_uint16array(a: &js_sys::Uint16Array) -> Vec<u16> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_uint8array(a: &js_sys::Uint8Array) -> Vec<u8> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_float64array(a: &js_sys::Float64Array) -> Vec<f64> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn bubble_sort_float32array(a: &js_sys::Float32Array) -> Vec<f32> {
    let mut v = a.to_vec();
    bubble_sort(&mut v);
    v
}

pub fn improved_bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
    let l = v.len();
    for j in 0..(l - 1) {
        let mut f = true;
        let mut min = j;
        for i in j..(l - j - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                f = false;
            }
            if v[i] < v[min] {
                min = i;
            }
        }
        if f {
            break;
        }
        if j != min {
            v.swap(j, min);
        }
    }
}

#[wasm_bindgen]
pub fn improved_bubble_sort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_int16array(a: &js_sys::Int16Array) -> Vec<i16> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_int8array(a: &js_sys::Int8Array) -> Vec<i8> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_uint32array(a: &js_sys::Uint32Array) -> Vec<u32> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_uint16array(a: &js_sys::Uint16Array) -> Vec<u16> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_uint8array(a: &js_sys::Uint8Array) -> Vec<u8> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_float64array(a: &js_sys::Float64Array) -> Vec<f64> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn improved_bubble_sort_float32array(a: &js_sys::Float32Array) -> Vec<f32> {
    let mut v = a.to_vec();
    improved_bubble_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_int16array(a: &js_sys::Int16Array) -> Vec<i16> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_int8array(a: &js_sys::Int8Array) -> Vec<i8> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_uint32array(a: &js_sys::Uint32Array) -> Vec<u32> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_uint16array(a: &js_sys::Uint16Array) -> Vec<u16> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_uint8array(a: &js_sys::Uint8Array) -> Vec<u8> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_float64array(a: &js_sys::Float64Array) -> Vec<f64> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn selection_sort_float32array(a: &js_sys::Float32Array) -> Vec<f32> {
    let mut v = a.to_vec();
    selection_sort(&mut v);
    v
}

#[wasm_bindgen]
pub fn merge_sort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_int16array(a: &js_sys::Int16Array) -> Vec<i16> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_int8array(a: &js_sys::Int8Array) -> Vec<i8> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_uint32array(a: &js_sys::Uint32Array) -> Vec<u32> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_uint16array(a: &js_sys::Uint16Array) -> Vec<u16> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_uint8array(a: &js_sys::Uint8Array) -> Vec<u8> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_float64array(a: &js_sys::Float64Array) -> Vec<f64> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}

#[wasm_bindgen]
pub fn merge_sort_float32array(a: &js_sys::Float32Array) -> Vec<f32> {
    let mut v = a.to_vec();
    merge_sort(&mut v)
}
