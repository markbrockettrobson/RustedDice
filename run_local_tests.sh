#!/bin/bash
set -e

echo "run cargo build"
( cargo build )

echo "run fmt to check formatting"
( cargo fmt )

echo "runing clippy to check linting and for common errors"
( cargo clippy --all-targets --all-features -- -D warnings )

echo "building docs"
( rustdoc src/lib.rs )

echo "runing tests"
( cargo test )

echo "runing coverage"
( cargo llvm-cov --html && cargo llvm-cov report )

echo "runing mutants, google mutation testing for details on why this is useful"
( cargo mutants -- --all-targets --all-features )
