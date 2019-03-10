#!/bin/bash

cd "$(dirname "$0")"/../crossbeam-queue
set -ex

export RUSTFLAGS="-D warnings"

cargo check --bins --examples --tests
cargo test

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
    # Check minimal versions.
    cargo update -Zminimal-versions
    cargo check
fi
