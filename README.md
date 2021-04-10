# rust-wasm-sort

## build
```shell
cargo build --target=wasm32-unknown-unknown --release
```

## build wasm for deno
```shell
wasm-bindgen --target deno target\wasm32-unknown-unknown\release\wasm_sort.wasm --out-dir ./pkg
```