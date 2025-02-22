.PHONY: all fmt clippy

all: clippy fmt

fmt:
	cargo fmt

clippy:
	cargo clippy --fix --allow-dirty
