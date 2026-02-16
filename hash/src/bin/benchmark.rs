use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::time::{Duration, Instant};

use hash::core::domain::hash_engine::HashEngine;

const WARMUP_ITERS: usize = 100;
const THROUGHPUT_ITERS: usize = 10;
const LATENCY_ITERS: usize = 500_000;

const THROUGHPUT_SIZES: &[(usize, &str)] = &[
    (1_000_000, "1 MB"),
    (10_000_000, "10 MB"),
    (100_000_000, "100 MB"),
];

const LATENCY_SIZES: &[(usize, &str)] = &[(8, "8 B"), (32, "32 B"), (256, "256 B"), (1024, "1 KB")];

fn generate_data(size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size];
    let mut state: u64 = 0xdeadbeef_cafebabe;
    for chunk in data.chunks_mut(8) {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let bytes = state.to_le_bytes();
        let len = chunk.len().min(8);
        chunk[..len].copy_from_slice(&bytes[..len]);
    }
    data
}

fn bench<H: Hasher + Default>(data: &[u8], iters: usize) -> Duration {
    for _ in 0..WARMUP_ITERS {
        let mut h = H::default();
        h.write(data);
        std::hint::black_box(h.finish());
    }

    let start = Instant::now();
    for _ in 0..iters {
        let mut h = H::default();
        h.write(data);
        std::hint::black_box(h.finish());
    }
    start.elapsed() / iters as u32
}

fn throughput_gbps(duration: Duration, size: usize) -> f64 {
    size as f64 / duration.as_secs_f64() / 1_000_000_000.0
}

fn main() {
    println!();
    println!("=== THROUGHPUT (GB/s) ===");
    println!(
        "{:<10} {:>14} {:>14}",
        "Size", "HashEngine", "DefaultHasher"
    );

    for &(size, label) in THROUGHPUT_SIZES {
        let data = generate_data(size);
        let ours = bench::<HashEngine>(&data, THROUGHPUT_ITERS);
        let theirs = bench::<DefaultHasher>(&data, THROUGHPUT_ITERS);

        println!(
            "{:<10} {:>11.3} GB/s {:>11.3} GB/s",
            label,
            throughput_gbps(ours, size),
            throughput_gbps(theirs, size),
        );
    }

    println!();
    println!("=== LATENCY (ns/op) ===");
    println!(
        "{:<10} {:>14} {:>14}",
        "Size", "HashEngine", "DefaultHasher"
    );

    for &(size, label) in LATENCY_SIZES {
        let data = generate_data(size);
        let ours = bench::<HashEngine>(&data, LATENCY_ITERS);
        let theirs = bench::<DefaultHasher>(&data, LATENCY_ITERS);

        println!(
            "{:<10} {:>11} ns {:>11} ns",
            label,
            ours.as_nanos(),
            theirs.as_nanos(),
        );
    }

    println!();
}
