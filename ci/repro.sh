#!/bin/bash

set -ex

cd crossbeam-epoch/examples/bench
cargo build --release
valgrind --tool=massif --stacks=yes --trace-children=yes target/release/stress-test
ms_print "$(ls massif.out.*)"
