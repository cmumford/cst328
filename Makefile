RELEASE=--release

.PHONY: default
default: build

.PHONY: clean
clean:
	cargo clean && cd examples/esp32 && cargo clean

.PHONY: build
build:
	cargo build --features=use_async

.PHONY: test
test:
	cargo +stable test --features=test

.PHONY: docs
docs:
	cargo doc --no-deps --document-private-items --open --features=use_async

.PHONY: clippy
clippy:
	cargo clippy --features=use_async -- -D clippy::pedantic

.PHONY: check
check:
	cargo check --features use_sync

.PHONY: machete
machete:
	cargo machete --with-metadata

.PHONY: publish
publish:
	cargo publish --features=use_async
