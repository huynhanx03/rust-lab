#![feature(portable_simd)]

pub mod constants;
pub mod core;
pub mod shared;

use crate::core::domain::hash_engine::HashEngine;
use std::hash::{Hash, Hasher};

pub fn hash<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut hasher = HashEngine::default();
    value.hash(&mut hasher);
    hasher.finish()
}
