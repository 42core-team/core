.PHONY: run build clean doc test

run:
	cargo run --bin game 10 20

build:
	cargo build --bin game

clean:
	cargo clean

re: clean build

doc:
	cargo doc --no-deps

test:
	cargo test
