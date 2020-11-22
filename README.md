# chainx-2.0-genesis-generator

This repo is used to prepare the genesis parameters for ChainX 2.0 mainnet, which is composed of two components:

1. [A Rust tool](Cargo.toml) for exporting and verifying the state of ChainX 1.0 by the given block height.

2. [A script in Node.js](genesis-params-builder) for processing on the exported 1.0 state and producing a JSON file for `genesis-builder` module of [ChainX](https://github.com/chainx-org/ChainX).

## Exporter and Verifier of ChainX 1.0 state

### Build

```bash
$ cargo build --release
```

### Usage

```bash
# Run the script
# the height currently only supports multiples of 10_000, like 20_000, 2_000_000
$ ./export.sh [height]

# Or edit the config and run the binaries manually
$ RUST_LOG=info cargo run --release --bin session-index
$ RUST_LOG=info cargo run --release --bin accounts
$ RUST_LOG=info cargo run --release --bin intentions
$ RUST_LOG=info cargo run --release --bin assets
$ RUST_LOG=info cargo run --release --bin assets-verify
$ RUST_LOG=info cargo run --release --bin deposit-weight
$ RUST_LOG=info cargo run --release --bin vote-weight
$ RUST_LOG=info cargo run --release --bin vote-weight-verify
```

## Genesis params builder

This script will extract and reorganize the 1.0 state to make the integration of `genesis-builder` module of ChainX 2.0 easier.

```bash
$ cd genesis-params-builder
# Install the dependencies
$ yarn
# The generated `res/2.0/genesis_builder_params.json` will be used in ChainX 2.0 genesis initialization.
$ node index.js
```

## License

[GPL-v3](LICENSE)
