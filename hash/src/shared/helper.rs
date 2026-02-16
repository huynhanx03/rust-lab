use std::simd::u64x4;

use crate::constants::{PRIME_DIFFUSE, PRIME_MIX};

pub fn round(accumulator: u64, input: u64) -> u64 {
    let mut acc = accumulator.wrapping_add(input.wrapping_mul(PRIME_MIX));
    acc = acc.rotate_left(31);
    acc.wrapping_mul(PRIME_DIFFUSE)
}

pub fn load_u64(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes.try_into().unwrap())
}

pub fn load_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.try_into().unwrap())
}

pub fn rotate_left_simd(acc: u64x4, shift: u32) -> u64x4 {
    (acc << u64x4::splat(shift as u64)) | (acc >> u64x4::splat(64 - shift as u64))
}

pub fn round_simd(acc: u64x4, input: u64x4) -> u64x4 {
    let prime_mix = u64x4::splat(PRIME_MIX);
    let prime_diffuse = u64x4::splat(PRIME_DIFFUSE);
    let mut acc = acc + input * prime_mix;
    acc = rotate_left_simd(acc, 31);
    acc * prime_diffuse
}

pub fn load_stripe(bytes: &[u8]) -> u64x4 {
    u64x4::from_array([
        load_u64(&bytes[0..8]),
        load_u64(&bytes[8..16]),
        load_u64(&bytes[16..24]),
        load_u64(&bytes[24..32]),
    ])
}