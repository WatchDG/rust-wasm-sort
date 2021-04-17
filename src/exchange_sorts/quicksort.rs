use sort::quicksort;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn quicksort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_int16array(a: &js_sys::Int16Array) -> Vec<i16> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_int8array(a: &js_sys::Int8Array) -> Vec<i8> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_uint32array(a: &js_sys::Uint32Array) -> Vec<u32> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_uint16array(a: &js_sys::Uint16Array) -> Vec<u16> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_uint8array(a: &js_sys::Uint8Array) -> Vec<u8> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_float64array(a: &js_sys::Float64Array) -> Vec<f64> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}

#[wasm_bindgen]
pub fn quicksort_float32array(a: &js_sys::Float32Array) -> Vec<f32> {
    let mut v = a.to_vec();
    quicksort(&mut v);
    v
}
