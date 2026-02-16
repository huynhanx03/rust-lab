use std::collections::HashSet;
use std::hash::Hasher;

use hash::core::domain::hash_engine::HashEngine;

const AVALANCHE_SAMPLES: usize = 10_000;
const AVALANCHE_INPUT_LEN: usize = 32;
const COLLISION_COUNT: usize = 1_000_000;
const DISTRIBUTION_COUNT: usize = 1_000_000;
const DISTRIBUTION_BUCKETS: usize = 1024;

struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn fill_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            let len = chunk.len().min(8);
            chunk[..len].copy_from_slice(&bytes[..len]);
        }
    }
}

fn hash_bytes(data: &[u8]) -> u64 {
    let mut h = HashEngine::default();
    h.write(data);
    h.finish()
}

fn test_avalanche() {
    println!("=== AVALANCHE TEST ===");

    let mut rng = Rng::new(0xdeadbeef12345678);
    let total_input_bits = AVALANCHE_INPUT_LEN * 8;
    let mut total_flipped: u64 = 0;
    let mut total_tests: u64 = 0;
    let mut min_pct: f64 = 100.0;
    let mut max_pct: f64 = 0.0;

    for _ in 0..AVALANCHE_SAMPLES {
        let mut input = vec![0u8; AVALANCHE_INPUT_LEN];
        rng.fill_bytes(&mut input);
        let original = hash_bytes(&input);

        for bit in 0..total_input_bits {
            input[bit / 8] ^= 1 << (bit % 8);
            let flipped = hash_bytes(&input);
            input[bit / 8] ^= 1 << (bit % 8);

            let changed = (original ^ flipped).count_ones() as u64;
            total_flipped += changed;
            total_tests += 1;

            let pct = changed as f64 / 64.0 * 100.0;
            min_pct = min_pct.min(pct);
            max_pct = max_pct.max(pct);
        }
    }

    let avg = total_flipped as f64 / total_tests as f64 / 64.0 * 100.0;
    let deviation = (avg - 50.0).abs();

    println!("  Samples:    {}", AVALANCHE_SAMPLES);
    println!("  Avg change: {:.2}%", avg);
    println!("  Min/Max:    {:.2}% / {:.2}%", min_pct, max_pct);
    println!("  Deviation:  {:.2}% from ideal 50%", deviation);

    let result = if deviation < 5.0 {
        "PASS"
    } else if deviation < 10.0 {
        "MARGINAL"
    } else {
        "FAIL"
    };
    println!("  Result:     {}", result);
    println!();
}

fn test_collisions() {
    println!("=== COLLISION TEST ===");

    let mut seen = HashSet::with_capacity(COLLISION_COUNT);
    let mut collisions = 0u64;

    for i in 0..COLLISION_COUNT {
        let key = format!("key-{}", i);
        let h = hash_bytes(key.as_bytes());
        if !seen.insert(h) {
            collisions += 1;
        }
    }

    println!("  Keys:       {}", COLLISION_COUNT);
    println!("  Unique:     {}", seen.len());
    println!("  Collisions: {}", collisions);
    println!(
        "  Rate:       {:.6}%",
        collisions as f64 / COLLISION_COUNT as f64 * 100.0
    );

    let result = if collisions == 0 { "PASS" } else { "FAIL" };
    println!("  Result:     {}", result);
    println!();
}

fn test_distribution() {
    println!("=== DISTRIBUTION TEST ===");

    let mut buckets = vec![0u64; DISTRIBUTION_BUCKETS];

    for i in 0..DISTRIBUTION_COUNT {
        let key = format!("dist-key-{}", i);
        let h = hash_bytes(key.as_bytes());
        buckets[(h as usize) % DISTRIBUTION_BUCKETS] += 1;
    }

    let expected = DISTRIBUTION_COUNT as f64 / DISTRIBUTION_BUCKETS as f64;
    let chi_sq: f64 = buckets
        .iter()
        .map(|&c| {
            let d = c as f64 - expected;
            d * d / expected
        })
        .sum();

    let dof = (DISTRIBUTION_BUCKETS - 1) as f64;
    let z = (2.0 * chi_sq).sqrt() - (2.0 * dof - 1.0).sqrt();

    let min = *buckets.iter().min().unwrap();
    let max = *buckets.iter().max().unwrap();

    println!("  Buckets:    {}", DISTRIBUTION_BUCKETS);
    println!("  Expected:   {:.1} per bucket", expected);
    println!("  Min/Max:    {} / {}", min, max);
    println!("  Chi-sq:     {:.2}", chi_sq);
    println!("  Z-score:    {:.4}", z);

    let result = if z.abs() < 2.0 {
        "PASS"
    } else if z.abs() < 3.0 {
        "MARGINAL"
    } else {
        "FAIL"
    };
    println!("  Result:     {}", result);
    println!();
}

fn main() {
    println!();
    test_avalanche();
    test_collisions();
    test_distribution();
}
