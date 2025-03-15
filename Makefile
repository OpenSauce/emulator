.PHONY: all fmt clippy test cpu-test

all: clippy fmt test

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --fix --allow-dirty

cpu-test:
	cargo run -- roms/blargg/cpu_instrs/cpu_instrs.gb
