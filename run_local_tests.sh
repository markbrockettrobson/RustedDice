#!/bin/bash
set -e

echo "run fmt"
( cd rusted_dice && cargo fmt)

echo "runing clippy"
( cd rusted_dice && cargo clippy --all-targets --all-features -- -D warnings )

echo "building docs"
(cd rusted_dice && rustdoc src/lib.rs )

echo "runing doctest"
( cd rusted_dice && cargo test --doc . )

echo "runing coverage"
( cd rusted_dice && cargo llvm-cov --html && cargo llvm-cov --no-run )

echo "runing mutants"
( cd rusted_dice && cargo mutants -- --all-targets --all-features )
