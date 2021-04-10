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

pub fn merge_sort<T: PartialOrd + Clone>(v: &mut [T]) -> (Vec<T>, usize) {
    let l = v.len();
    if l == 1 {
        return (v.to_vec(), 1);
    }
    let m = (l as f64 / 2.).floor() as usize;
    let (lp, rp) = v.split_at_mut(m);
    let (mut lv, lvl) = merge_sort(lp);
    let (mut rv, rvl) = merge_sort(rp);
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
    return (new_v, l);
}

#[wasm_bindgen]
pub fn merge_sort_int32array(a: &js_sys::Int32Array) -> Vec<i32> {
    let mut v = a.to_vec();
    merge_sort(&mut v).0
}