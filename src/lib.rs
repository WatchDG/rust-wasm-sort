mod exchange_sorts;
mod insertion_sorts;
mod merge_sorts;
mod selection_sorts;
mod hybrid_sorts;

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

pub use exchange_sorts::quicksort_float32array;
pub use exchange_sorts::quicksort_float64array;
pub use exchange_sorts::quicksort_int16array;
pub use exchange_sorts::quicksort_int32array;
pub use exchange_sorts::quicksort_int8array;
pub use exchange_sorts::quicksort_uint16array;
pub use exchange_sorts::quicksort_uint32array;
pub use exchange_sorts::quicksort_uint8array;

pub use selection_sorts::selection_sort_float32array;
pub use selection_sorts::selection_sort_float64array;
pub use selection_sorts::selection_sort_int16array;
pub use selection_sorts::selection_sort_int32array;
pub use selection_sorts::selection_sort_int8array;
pub use selection_sorts::selection_sort_uint16array;
pub use selection_sorts::selection_sort_uint32array;
pub use selection_sorts::selection_sort_uint8array;

pub use selection_sorts::heapsort_float32array;
pub use selection_sorts::heapsort_float64array;
pub use selection_sorts::heapsort_int16array;
pub use selection_sorts::heapsort_int32array;
pub use selection_sorts::heapsort_int8array;
pub use selection_sorts::heapsort_uint16array;
pub use selection_sorts::heapsort_uint32array;
pub use selection_sorts::heapsort_uint8array;

pub use insertion_sorts::insertion_sort_float32array;
pub use insertion_sorts::insertion_sort_float64array;
pub use insertion_sorts::insertion_sort_int16array;
pub use insertion_sorts::insertion_sort_int32array;
pub use insertion_sorts::insertion_sort_int8array;
pub use insertion_sorts::insertion_sort_uint16array;
pub use insertion_sorts::insertion_sort_uint32array;
pub use insertion_sorts::insertion_sort_uint8array;

pub use hybrid_sorts::introsort_float32array;
pub use hybrid_sorts::introsort_float64array;
pub use hybrid_sorts::introsort_int16array;
pub use hybrid_sorts::introsort_int32array;
pub use hybrid_sorts::introsort_int8array;
pub use hybrid_sorts::introsort_uint16array;
pub use hybrid_sorts::introsort_uint32array;
pub use hybrid_sorts::introsort_uint8array;