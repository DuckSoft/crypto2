#![cfg_attr(test, feature(test))]
// #![feature(stdsimd, llvm_asm)]
#![allow(unused_macros, unused_assignments)]

#[cfg(test)]
extern crate test;

#[cfg(test)]
extern crate hex;


mod util;
pub mod mem;


// cryptographic hash function (CHF)
pub mod hash;

// Key derivation function (KDF)
pub mod kdf;

pub mod mac;

pub mod blockmode;

pub mod blockcipher;
pub mod streamcipher;
pub mod aeadcipher;


#[cfg(feature = "openssh")]
pub mod openssh;


#[cfg(target_arch = "aarch64")]
pub(crate) mod aarch64 {
    pub type p64 = u64;

    /// ARM-specific 128-bit wide vector of sixteen packed `u8`.
    #[repr(simd)]
    #[derive(Debug, Clone, Copy)]
    pub struct uint8x16_t(
        u8, u8 ,u8, u8, u8, u8 ,u8, u8,
        u8, u8 ,u8, u8, u8, u8 ,u8, u8,
    );

    /// ARM-specific 128-bit wide vector of four packed `u32`.
    #[repr(simd)]
    #[derive(Debug, Clone, Copy)]
    pub struct uint32x4_t(u32, u32, u32, u32);

    /// ARM-specific 128-bit wide vector of two packed `u64`.
    #[repr(simd)]
    #[derive(Debug, Clone, Copy)]
    pub struct uint64x2_t(u64, u64);

    /// ARM-specific 128-bit wide vector of two packed `p64`.
    #[repr(simd)]
    #[derive(Debug, Clone, Copy)]
    pub struct poly64x2_t(p64, p64);


    #[link(name = "libneon")]
    extern "C" {
        fn vrbitq_u8(a: uint8x16_t) -> uint8x16_t;
        fn vreinterpretq_u64_u8(a: uint8x16_t) -> uint64x2_t;
        fn vgetq_lane_u64(v: uint64x2_t, imm5: i32) -> u64;
        fn vmull_p64(a: u64, b: u64) -> u128;
        fn veorq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
        fn vdupq_n_u8(value: u8) -> uint8x16_t;
        fn vextq_u8(a: uint8x16_t, b: uint8x16_t, n: i32) -> uint8x16_t;
        fn vld1q_u8(ptr: *const u8) -> uint8x16_t;
        fn vaeseq_u8(data: uint8x16_t, key: uint8x16_t) -> uint8x16_t;
        fn vaesmcq_u8(data: uint8x16_t) -> uint8x16_t;
        fn vaesimcq_u8(data: uint8x16_t) -> uint8x16_t;
        fn vaesdq_u8(data: uint8x16_t, key: uint8x16_t) -> uint8x16_t;
        fn vandq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;

        fn vaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
        fn vsha256su0q_u32(w0_3: uint32x4_t, w4_7: uint32x4_t) -> uint32x4_t;
        fn vsha256hq_u32(hash_abcd: uint32x4_t, hash_efgh: uint32x4_t, wk: uint32x4_t) -> uint32x4_t;
        fn vsha256h2q_u32(hash_efgh: uint32x4_t, hash_abcd: uint32x4_t, wk: uint32x4_t) -> uint32x4_t;
        fn vsha256su1q_u32(tw0_3: uint32x4_t, w8_11: uint32x4_t, w12_15: uint32x4_t) -> uint32x4_t;
    }
}