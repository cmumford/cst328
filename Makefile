RELEAES=
RELEASE=--release

.PHONY: default
default: examples-all

.PHONY: simple-esp32c3
simple-esp32c3:
	cargo build --features=esp32c3,use_sync --target riscv32imc-unknown-none-elf $(RELEASE) --example simple

.PHONY: simple-esp32c6
simple-esp32c6:
	cargo build --features=esp32c6,use_sync --target riscv32imac-unknown-none-elf $(RELEASE) --example simple

.PHONY: simple-esp32h2
simple-esp32h2:
	cargo build --features=esp32h2,use_sync --target riscv32imac-unknown-none-elf $(RELEASE) --example simple

.PHONY: simple-esp32
simple-esp32:
	cargo build --features=esp32,use_sync --target xtensa-esp32-none-elf $(RELEASE) --example simple

.PHONY: simple-esp32s2
simple-esp32s2:
	cargo build --features=esp32s2,use_sync --target xtensa-esp32s2-none-elf $(RELEASE) --example simple

.PHONY: simple-esp32s3
simple-esp32s3:
	cargo build --features=esp32s3,use_sync --target xtensa-esp32s3-none-elf $(RELEASE) --example simple

.PHONY: simple-esp32c3-async
simple-esp32c3-async:
	cargo build --features=esp32c3,use_async --target riscv32imc-unknown-none-elf $(RELEASE) --example simple_async

.PHONY: simple-esp32c6-async
simple-esp32c6-async:
	cargo build --features=esp32c6,use_async --target riscv32imac-unknown-none-elf $(RELEASE) --example simple_async

.PHONY: simple-esp32h2-async
simple-esp32h2-:
	cargo build --features=esp32h2,use_async --target riscv32imac-unknown-none-elf $(RELEASE) --example simple_async

.PHONY: simple-esp32-async
simple-esp32-async:
	cargo build --features=esp32,use_async --target xtensa-esp32-none-elf $(RELEASE) --example simple_async

.PHONY: simple-esp32s2-async
simple-esp32s2-async:
	cargo build --features=esp32s2,use_async --target xtensa-esp32s2-none-elf $(RELEASE) --example simple_async

.PHONY: simple-esp32s3-async
simple-esp32s3-async:
	cargo build --features=esp32s3,use_async --target xtensa-esp32s3-none-elf $(RELEASE) --example simple_async

.PHONY: examples-sync
examples-sync: simple-esp32c3 simple-esp32c6 simple-esp32h2 simple-esp32 simple-esp32s2 simple-esp32s3

.PHONY: examples-async
examples-async: simple-esp32c3-async simple-esp32c6-async simple-esp32h2-async simple-esp32-async simple-esp32s2-async simple-esp32s3-async

.PHONY: examples-all
examples-all:examples-sync examples-async

.PHONY: flash-simple-esp32c6
flash-simple-esp32c6:
	cargo espflash flash --example simple --monitor --baud=921600 --target riscv32imac-unknown-none-elf $(RELEASE) --features=esp32c6,use_sync

.PHONY: flash-simple-esp32c6-async
flash-simple-esp32c6-async:
	cargo espflash flash --example simple_async --monitor --baud=921600 --target riscv32imac-unknown-none-elf $(RELEASE) --features=esp32c6,use_async

.PHONY: clean
clean:
	cargo clean

.PHONY: docs
docs:
	cargo doc --no-deps --document-private-items --open --features=esp32s3,use_async

.PHONY: clippy
clippy:
	cargo clippy --features=esp32s3,use_async -- -D clippy::pedantic

.PHONY: check
check:
	cargo check --target riscv32imac-unknown-none-elf --features esp32c6,use_sync --target riscv32imc-unknown-none-elf

.PHONY: machete
machete:
	cargo machete --with-metadata
