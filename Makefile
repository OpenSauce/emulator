.PHONY: all fmt clippy test

all: clippy fmt test

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --fix --allow-dirty
