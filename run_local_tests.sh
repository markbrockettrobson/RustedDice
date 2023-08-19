#!/bin/bash
set -e

echo "run fmt to check formatting"
( cd rusted_dice && cargo fmt)

echo "runing clippy to check linting and for common errors"
( cd rusted_dice && cargo clippy --all-targets --all-features -- -D warnings )

echo "building docs"
(cd rusted_dice && rustdoc src/lib.rs )

echo "runing doctest"
( cd rusted_dice && cargo test --doc . )

echo "runing coverage"
( cd rusted_dice && cargo llvm-cov --html && cargo llvm-cov --no-run )

echo "runing mutants, google mutation testing for details on why this is useful"
( cd rusted_dice && cargo mutants -- --all-targets --all-features )
