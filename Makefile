.PHONY: all fmt clippy

all: fmt clippy

fmt:
	cargo fmt

clippy:
	cargo clippy --fix --allow-dirty
