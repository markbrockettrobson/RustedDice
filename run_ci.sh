echo "runing clippy"
( cd rusted_dice && cargo clippy --fix --all-targets --all-features -- -D warnings )

echo "runing test dockerfile"
docker build . --file ContinuousIntegration/Test.dockerfile --tag rusted_dice_local_build