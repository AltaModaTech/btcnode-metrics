# btcnode-metrics

The best Bitcoin node metrics exporter for Prometheus based on [corepc-client](https://crates.io/crates/corepc-client).

## Install & Configure

### Install

`cargo install btcnode-metrics`

### Configure

After installing btcnode-metrics,

- Copy `config.toml.example` to `config.local.toml`.
- Edit `config.local.toml`
  - the _node_ section is for the Bitcoin node to monitor
  - the _server_ section is for exposing the endpoint for Prometheus

## Usage

To run:

`cargo run -- -c ./config.local.toml`

For additional output, set the [logging level(https://docs.rs/env_logger/latest/env_logger/)]:

`RUST_LOG=info cargo run -- -c ./config.local.toml`

## Additional Details

### About corepc-client

The [corepc-client](https://crates.io/crates/corepc-client) crate replaces the previous the Bitcoin node RPC crates that were archived on 11/25/2025:

- [bitcoincore-rpc](https://crates.io/crates/bitcoincore-rpc) [GitHub](https://github.com/rust-bitcoin/rust-bitcoincore-rpc)
- [bitcoincore-rpc-json](https://crates.io/crates/) [GitHub](https://github.com/rust-bitcoin/rust-bitcoincore-rpc/tree/master/json)

The Rust Bitcoin Community's public repository for this crate is at [corepc-client](https://github.com/rust-bitcoin/corepc).

### Code structure

This repository's code is separated into:

- _btcnode-metrics_ implements the API for Prometheus to call for gathering metrics.
- _btcnode-metrics_ gathers metrics from the Bitcoin node and transforms them into Prometheus format.
