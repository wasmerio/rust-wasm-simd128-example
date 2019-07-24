# Rust Wasm SIMD 128 example

This repo

## Build

We build this crate using `nightly-2019-05-20` because we wanted to use a "stable" version of nightly.

```bash
# Add rust nightly (last versions can't compile 😅)
rustup toolchain install nightly-2019-05-20

# Add wasm32-wasi to the toolchain
rustup target add wasm32-wasi --toolchain=nightly-2019-05-20

# Build the instance
RUSTFLAGS='-C target-feature=+simd128' cargo +nightly-2019-05-20 build --target=wasm32-wasi

# Verify that it's working!
wasm2wat --enable-simd ./target/wasm32-wasi/debug/rust-wasm-simd128-example.wasm
```

## Run it!

```
wasmer-release run --backend=llvm ./target/wasm32-wasi/debug/rust-wasm-simd128-example.wasm

# TADA!
```
