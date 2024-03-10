#!/bin/bash
set -e

echo "rustup default to nightly"
( rustup default nightly )

echo "rustup update"
( rustup update )

echo "rustup component add rustfmt"
( rustup component add rustfmt )

echo "rustup component add clippy"
( rustup component add clippy )

echo "cargo install cargo-llvm-cov"
( cargo install cargo-llvm-cov )

echo "cargo install --locked cargo-mutants"
( cargo install --locked cargo-mutants )

echo "cargo build"
( cargo build )
