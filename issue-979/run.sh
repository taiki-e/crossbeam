#!/usr/bin/env bash
set -euxo pipefail
IFS=$'\n\t'

cargo clean && cargo "$@" build -q --release && nm -C target/release/repro | grep flavor | (grep -v list || true)
cargo clean && cargo "$@" build -q --release --features array && nm -C target/release/repro | grep flavor | (grep -v array || true)
