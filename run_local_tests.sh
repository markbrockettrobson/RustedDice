set -e

echo "runing clippy"
( cd rusted_dice && cargo clippy --all-targets --all-features -- -D warnings )

echo "runing mutants"
( cd rusted_dice && cargo mutants -- --all-targets --all-features )

echo "runing coverage"
( cd rusted_dice && cargo llvm-cov )
