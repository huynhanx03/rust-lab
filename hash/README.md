# Hash Engine

A custom xxHash-style hash engine built in Rust for learning purposes. Features multi-accumulator parallel processing with SIMD, streaming buffer management, and avalanche finalization.

## Design

- Multi-accumulator parallel processing with SIMD
- Block-based streaming with internal buffering
- Avalanche finalization for full bit diffusion
- Graceful fallback for small inputs

## Requirements

- Rust **nightly** (for `#![feature(portable_simd)]`)

```bash
rustup override set nightly
```

## Usage

```bash
make run       # Run demo
make bench     # Throughput + latency benchmark (release mode)
make quality   # Avalanche, collision, distribution tests (release mode)
make clean     # Clean build artifacts
```

## Benchmark Results

Compared against `std::collections::hash_map::DefaultHasher` (SipHash):

### Throughput

| Size   | HashEngine | DefaultHasher |
|--------|-----------|---------------|
| 1 MB   | 5.05 GB/s | 3.19 GB/s     |
| 10 MB  | 4.99 GB/s | 3.19 GB/s     |
| 100 MB | 4.99 GB/s | 3.23 GB/s     |

**~1.55x faster** than DefaultHasher across all sizes.

### Latency

| Size  | HashEngine | DefaultHasher |
|-------|-----------|---------------|
| 8 B   | 5 ns      | 9 ns          |
| 32 B  | 8 ns      | 13 ns         |
| 256 B | 32 ns     | 74 ns         |
| 1 KB  | 159 ns    | 314 ns        |

**1.6–2.3x faster** for small payloads.

## Quality Results

| Test         | Result | Detail                        |
|--------------|--------|-------------------------------|
| Avalanche    | PASS   | 50.00% avg (ideal = 50%)      |
| Collision    | PASS   | 0 collisions in 1M keys       |
| Distribution | PASS   | Z-score = -0.32 (uniform)     |
