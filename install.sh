#!/bin/bash
set -e

echo "rustup default to nightly"
( rustup default nightly )

echo "rustup component add rustfmt"
( rustup component add rustfmt )

echo "rustup component add clippy"
( rustup component add clippy )

echo "cargo install cargo-llvm-cov"
( cargo install cargo-llvm-cov )

echo "cargo install mutants"
( cargo install cargo-mutants )

echo "cargo build"
( cargo build )
