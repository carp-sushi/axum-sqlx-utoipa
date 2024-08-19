.PHONY: all
all: fmt build test lint openapi

.PHONY: fmt
fmt:
	@cargo fmt --all

.PHONY: check
check:
	@cargo check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: itest
itest:
	@RUST_LOG=off cargo test -- --ignored

.PHONY: lint
lint:
	@cargo clippy

.PHONY: clean
clean:
	@cargo clean

.PHONY: run
run:
	@mkdir -p .storage
	@cargo run --bin sqlx-todos

.PHONY: release
release:
	@cargo build --release

.PHONY: openapi
openapi:
	@cargo run --bin openapi > docs/openapi.json
