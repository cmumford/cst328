RELEASE=--release
RELEAES=

.PHONY: build
build:
	cargo build $(RELEASE)

.PHONY: clean
clean:
	cargo clean
