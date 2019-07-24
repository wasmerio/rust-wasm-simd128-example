//! A simple slab allocator for pages in wasm

#![feature(stdsimd)]
// #![cfg(target_feature = "simd128")]
#![cfg(target_arch = "wasm32")]

extern crate core_arch;

use core_arch::wasm32::*;

fn heyx(x: v128, y: v128) -> v128 {
    i32x4_mul(x, y)
}

fn main() {
    let a: [u32; 4] = [0; 4];
    let b: [u32; 4] = [0; 4];

    unsafe {
        let vec_a: v128 = ::std::mem::transmute(a);
        let vec_b: v128 = ::std::mem::transmute(b);
        let vec_c: v128 = heyx(vec_a, vec_b);
        dbg!(vec_c);
    }

}
