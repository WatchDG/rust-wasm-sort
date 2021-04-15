mod exchange_sorts;
mod merge_sorts;
mod selection_sorts;

extern crate js_sys;
extern crate sort;
extern crate wasm_bindgen;
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use merge_sorts::merge_sort_float32array;
pub use merge_sorts::merge_sort_float64array;
pub use merge_sorts::merge_sort_int16array;
pub use merge_sorts::merge_sort_int32array;
pub use merge_sorts::merge_sort_int8array;
pub use merge_sorts::merge_sort_uint16array;
pub use merge_sorts::merge_sort_uint32array;
pub use merge_sorts::merge_sort_uint8array;

pub use exchange_sorts::bubble_sort_float32array;
pub use exchange_sorts::bubble_sort_float64array;
pub use exchange_sorts::bubble_sort_int16array;
pub use exchange_sorts::bubble_sort_int32array;
pub use exchange_sorts::bubble_sort_int8array;
pub use exchange_sorts::bubble_sort_uint16array;
pub use exchange_sorts::bubble_sort_uint32array;
pub use exchange_sorts::bubble_sort_uint8array;
