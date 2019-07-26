# Rust Wasm SIMD 128 example

This repo is made to showcase how to emit Wasm SIMD 128 instructions from Rust, and use it with [Wasmer](https://github.com/wasmerio/wasmer).

## Build

We build this crate using `nightly-2019-05-20` because we wanted to use a "stable" version of nightly.

```bash
# Initialize stdarch
git submodule update --init
```


```bash
# Add rust nightly (last versions can't compile 😅)
rustup toolchain install nightly-2019-05-20

# Add wasm32-wasi to the toolchain
rustup target add wasm32-wasi --toolchain=nightly-2019-05-20

# Build the instance
RUSTFLAGS='-C target-feature=+simd128' cargo +nightly-2019-05-20 build --release --target=wasm32-wasi

# Verify that it's working!
wasm2wat --enable-simd ./target/wasm32-wasi/release/rust-wasm-simd128-example.wasm
```

## Run it!

```
wasmer-release run --backend=llvm ./target/wasm32-wasi/release/rust-wasm-simd128-example.wasm

# TADA!
```
