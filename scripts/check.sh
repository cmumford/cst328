#!/bin/sh -e

# Check that everything is good. Recommended to run
# before pushing changes.

cargo fmt --all -- --check
make check
make clippy
make
make test

cd examples/esp32
cargo fmt --all -- --check
make check
make clippy
make