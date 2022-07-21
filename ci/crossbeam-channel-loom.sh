#!/bin/bash

cd "$(dirname "$0")"/../crossbeam-channel
set -ex

export RUSTFLAGS="${RUSTFLAGS:-} --cfg crossbeam_loom --cfg crossbeam_sanitize"

# With MAX_PREEMPTIONS=2 the loom tests (currently) take around 11m.
# If we were to run with =3, they would take several times that,
# which is probably too costly for CI.
LOOM_MAX_PREEMPTIONS=2 \
    LOOM_LOG=1 \
    LOOM_LOCATION=1 \
    cargo test --lib --release --features loom -- --nocapture
