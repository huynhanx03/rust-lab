#![allow(dead_code, unused_imports, dead_code)]

use std::{
    hash::{BuildHasher, Hasher},
    simd::u64x4,
};

use crate::{
    constants::{PRIME_DIFFUSE, PRIME_FINAL, PRIME_LANE, PRIME_MIX, PRIME_TAIL},
    shared::helper::{load_stripe, load_u32, load_u64, round, round_simd},
};

const BASE: u64 = 0xcafecafecafecafe;

pub struct HashEngine {
    accumulators: u64x4,
    buffer: [u8; 32],
    buffer_len: usize,
    bytes_done: usize,
}

impl HashEngine {
    pub fn new(seed: u64) -> Self {
        let accumulators = u64x4::from_array([
            seed.wrapping_add(PRIME_DIFFUSE).wrapping_add(PRIME_MIX),
            seed.wrapping_add(PRIME_MIX),
            seed,
            seed.wrapping_add(PRIME_DIFFUSE),
        ]);

        Self {
            accumulators,
            buffer: [0; 32],
            buffer_len: 0,
            bytes_done: 0,
        }
    }
}

impl Default for HashEngine {
    fn default() -> Self {
        Self::new(BASE)
    }
}

impl Hasher for HashEngine {
    fn write(&mut self, bytes: &[u8]) {
        self.bytes_done += bytes.len();
        let mut offset = 0;

        // The buffer is used to store the remaining bytes
        // that are not enough to form a 32-byte block.
        if self.buffer_len > 0 {
            let remaining = 32 - self.buffer_len;
            let take = remaining.min(bytes.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&bytes[..take]);
            self.buffer_len += take;
            offset += take;

            if self.buffer_len < 32 {
                return;
            }

            let input = load_stripe(&self.buffer);
            self.accumulators = round_simd(self.accumulators, input);
            self.buffer_len = 0;
        }

        // Process the remaining bytes in 32-byte chunks.
        let remaining = &bytes[offset..];
        let chunks = remaining.chunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let input = load_stripe(chunk);
            self.accumulators = round_simd(self.accumulators, input);
        }

        self.buffer_len = remainder.len();
        self.buffer[..self.buffer_len].copy_from_slice(remainder);
    }

    fn finish(&self) -> u64 {
        let mut state: u64;

        // If the number of bytes processed is greater than or equal to 32,
        // we can use the accumulators to compute the hash.
        if self.bytes_done >= 32 {
            state = self.accumulators[0]
                .rotate_left(1)
                .wrapping_add(self.accumulators[1].rotate_left(7))
                .wrapping_add(self.accumulators[2].rotate_left(12))
                .wrapping_add(self.accumulators[3].rotate_left(18));

            for i in 0..4 {
                state ^= round(0, self.accumulators[i]);
                state = state.wrapping_mul(PRIME_DIFFUSE).wrapping_add(PRIME_FINAL);
            }
        } else {
            state = self.accumulators[2].wrapping_add(PRIME_TAIL);
        }

        // Add the number of bytes processed to the state.
        state = state.wrapping_add(self.bytes_done as u64);

        // Process the remaining bytes in 32-byte chunks.
        let buf = &self.buffer[..self.buffer_len];
        let mut pos = 0;

        while pos + 8 <= buf.len() {
            state ^= round(0, load_u64(&buf[pos..pos + 8]));
            state = state
                .rotate_left(27)
                .wrapping_mul(PRIME_DIFFUSE)
                .wrapping_add(PRIME_FINAL);
            pos += 8;
        }

        while pos + 4 <= buf.len() {
            let k = load_u32(&buf[pos..pos + 4]) as u64;
            state ^= k.wrapping_mul(PRIME_DIFFUSE);
            state = state
                .rotate_left(23)
                .wrapping_mul(PRIME_MIX)
                .wrapping_add(PRIME_LANE);
            pos += 4;
        }

        while pos < buf.len() {
            state ^= (buf[pos] as u64).wrapping_mul(PRIME_TAIL);
            state = state.rotate_left(11).wrapping_mul(PRIME_DIFFUSE);
            pos += 1;
        }

        // Finalize the state.
        state ^= state >> 33;
        state = state.wrapping_mul(PRIME_MIX);
        state ^= state >> 29;
        state = state.wrapping_mul(PRIME_LANE);
        state ^= state >> 32;

        state
    }
}
