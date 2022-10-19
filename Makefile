lint:
	cargo fmt
	cargo clippy

test:
	cargo test

build:
	cargo build --release