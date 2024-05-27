CARGO_FLAGS = --all-targets --all-features

.PHONY: all check build run clippy test fmt clean

check-all: check clippy test fmt

check:
	cargo check

build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run

clippy:
	cargo clippy $(CARGO_FLAGS) -- -D warnings

test:
	cargo test

fmt:
	cargo fmt

clean:
	cargo clean

