# btcnode-metrics

The Bitcoin node metrics exporter for Prometheus based on Rust Bitcoin Community's crate for Bitcoin JSON-RPC support, [corepc-client](https://crates.io/crates/corepc-client). Why is that important? Other exporters use deprecated/archived crates like rust-bitcoincore-rpc or rust-bitcoind-json-rpc. Using this exporter helps you keep up to date with the Rust Bitcoin Community's work.

## Install & Configure

These instructions are for installing and running from the shell. To install as a SystemD service, follow [these instructions](./linux/SystemD-service.md).

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

The Rust Bitcoin Community's [corepc-client](https://crates.io/crates/corepc-client) crate replaces the previous the Bitcoin node RPC crates that were _archived on 11/25/2025_:

- [bitcoincore-rpc](https://crates.io/crates/bitcoincore-rpc) [GitHub](https://github.com/rust-bitcoin/rust-bitcoincore-rpc)
- [bitcoincore-rpc-json](https://crates.io/crates/) [GitHub](https://github.com/rust-bitcoin/rust-bitcoincore-rpc/tree/master/json)

### Code structure

This repository's code is separated into:

- _btcnode-metrics_ implements the API for Prometheus to call for gathering metrics.
- _btcnode_metrics_gatherer_ module gathers metrics from the Bitcoin node and transforms them into Prometheus format.
