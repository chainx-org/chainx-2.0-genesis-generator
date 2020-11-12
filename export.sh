#!/usr/bin/env bash

if [ $# -eq 0 ]; then
    echo "Usage: bash export.sh [height]"
    exit 1
fi

echo "========================================================================="
rustup -V
cargo -V
rustc -V
echo "Rust toolchain installed"
echo "Building binaries..."
cargo build --release
echo "========================================================================="

old_height=$(awk -F":" '/height/{print $2}' config.json)
new_height=$1
sed -e "s@$old_height@$new_height@g" -i config.json
echo "ChainX Block Height: $old_height (old) ==> $new_height (new)"

state_dir=$(pwd)/state_1.0/$new_height

echo "========================================================================="
echo "Collect ChainX accounts from storage via RPC..."
accounts_filename=$state_dir/accounts.json
if [ ! -f "$accounts_filename" ]; then
  RUST_LOG=info cargo run --release --bin accounts
fi
echo "Done"
echo "========================================================================="
echo "Get Intentions via RPC..."
intentions_filename=$state_dir/intentions.json
if [ ! -f "$intentions_filename" ]; then
  RUST_LOG=info cargo run --release --bin intentions
fi
echo "Done"
echo "========================================================================="
echo "Get Assets of ChainX accounts via RPC..."
assets_filename=$state_dir/assets.json
if [ ! -f "$assets_filename" ]; then
  RUST_LOG=info cargo run --release --bin assets
  echo "Verify the assets..."
  RUST_LOG=info cargo run --release --bin assets-verify
fi
echo "Done"
echo "========================================================================="
echo "Get deposit weight of ChainX accounts via RPC..."
deposit_weight_filename=$state_dir/deposit-weight-accounts.json
if [ ! -f "$deposit_weight_filename" ]; then
  RUST_LOG=info cargo run --release --bin deposit-weight
fi
echo "Done"
echo "========================================================================="
echo "Get vote weight of accounts and nodes via RPC..."
# It takes a long time to get vote weight, and it may fail due to unstable connection.
# At that time, you may need to manually run the corresponding binary.
vote_weight_filename=$state_dir/vote-weight-accounts.json
if [ ! -f "$vote_weight_filename" ]; then
  RUST_LOG=info cargo run --release --bin vote-weight
  echo "Verify the vote weight between accounts and nodes..."
  RUST_LOG=info cargo run --release --bin vote-weight-verify
fi
echo "Done"
echo "========================================================================="
echo "Finished"
