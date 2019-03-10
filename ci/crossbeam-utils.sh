#!/bin/bash

cd "$(dirname "$0")"/../crossbeam-utils
set -ex

export RUSTFLAGS="-D warnings"

cargo check --no-default-features
cargo check --bins --examples --tests
cargo test

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
    cargo check --no-default-features --features nightly
    cargo test --features nightly

    # Check minimal versions.
    cargo update -Zminimal-versions
    cargo check --all-features
fi
