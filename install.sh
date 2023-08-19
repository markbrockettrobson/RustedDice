#!/bin/bash
set -e

echo "rustup component add clippy"
( rustup component add clippy )

echo "cargo install cargo-llvm-cov"
( cd rusted_dice && cargo install cargo-llvm-cov )

echo "cargo install mutants"
( cd rusted_dice && cargo install cargo-mutants )

echo "cargo build"
( cd rusted_dice && cargo build )
