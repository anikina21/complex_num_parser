# Makefile for complex-num-parser
PACKAGE = complex-num-parser

.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run -- parse "(3+4i)*(1-2i)"

.PHONY: file
file:
	cargo run -- file examples.txt

.PHONY: cli-help
cli-help:
	cargo run -- help

.PHONY: credits
credits:
	cargo run -- credits

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings
