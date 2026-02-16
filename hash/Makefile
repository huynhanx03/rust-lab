.PHONY: run clean bench quality

run:
	cargo build
	./target/debug/hash

bench:
	cargo run --bin benchmark --release

quality:
	cargo run --bin quality --release

clean:
	cargo clean
