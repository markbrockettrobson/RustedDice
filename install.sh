#!/bin/bash
set -e

echo "rustup component add clippy"
( rustup component add clippy )

echo "cargo install cargo-llvm-cov"
( cargo install cargo-llvm-cov )

echo "cargo install mutants"
( cargo install cargo-mutants )

echo "cargo build"
( cargo build )
