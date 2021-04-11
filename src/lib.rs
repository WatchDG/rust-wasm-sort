extern crate js_sys;
extern crate wasm_bindgen;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
    let l = v.len();
    for j in 0..(l - 1) {
        for i in 0..(l - j - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
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

pub fn selection_sort<T: PartialOrd>(v: &mut Vec<T>) {
    let l = v.len();
    for i in 0..(l - 1) {
        let mut mi = i;
        for j in (i + 1)..l {
            if v[mi] > v[j] {
                mi = j;
            }
        }
        if i != mi {
            v.swap(i, mi);
        }
    }
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

pub fn merge_sort<T: PartialOrd + Clone>(v: &mut [T]) -> Vec<T> {
    let l = v.len();
    if l < 2 {
        return v.to_vec();
    }
    let m = (l as f64 / 2.).floor() as usize;
    let (lp, rp) = v.split_at_mut(m);
    let mut lv = merge_sort(lp);
    let mut rv = merge_sort(rp);
    let lvl = lv.len();
    let rvl = rv.len();
    let mut li = 0;
    let mut ri = 0;
    let mut new_v = Vec::with_capacity(l);
    for _ in 0..l {
        if lv[li] > rv[ri] {
            new_v.push(rv[ri].clone());
            ri += 1;
        } else {
            new_v.push(lv[li].clone());
            li += 1;
        }
        if li == lvl {
            new_v.extend_from_slice(rv.split_off(ri).as_slice());
            break;
        }
        if ri == rvl {
            new_v.extend_from_slice(lv.split_off(li).as_slice());
            break;
        }
    }
    return new_v;
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
