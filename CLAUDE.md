# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- Build: `cargo build`
- Test (what CI runs): `cargo test`; single test: `cargo test <name>` (tests live in `#[cfg(test)]` in `src/btcnode_metrics_gatherer/collector.rs`)
- Run: `RUST_LOG=info cargo run -- -c ./config.local.toml` (copy `config.toml.example` to `config.local.toml`; `*.local.toml` is gitignored; default config path is `config.toml`)

## Architecture

Single binary crate (edition 2024), a Prometheus exporter for a Bitcoin Core node using `corepc-client` (sync client, pinned to the `v29` node API types in `node.rs`). Bumping `corepc-client` usually means updating the `v29` imports in `node.rs` and the field accesses in `collector.rs`.

- `main.rs`: CLI (clap), config load, wires node -> collector -> service, serves axum routes `/metrics` and `/health`.
- `handlers.rs`: `/metrics` runs `service.scrape()` inside `spawn_blocking` because the RPC client is synchronous. Every scrape triggers live RPC calls to the node.
- `state.rs`: `AppState` holding `Arc<MetricsService<BitcoinNode>>`.
- `src/btcnode_metrics_gatherer/` (module, formerly a separate crate):
  - `node.rs`: `NodeClient` trait (one method per RPC) and `BitcoinNode` impl over corepc. The trait exists so the collector can be tested with mocks.
  - `collector.rs`: `MetricsCollector<N: NodeClient>` calls each RPC, sets gauges on `BitcoinMetrics`, and logs/continues on per-RPC failure (tracks `had_error`) rather than failing the scrape.
  - `metrics.rs`: `BitcoinMetrics` - the Prometheus registry and all gauge definitions. Add new metrics here, then set them in the collector.
  - `service.rs`: `MetricsService` runs `collect()` then encodes the registry to Prometheus text format.
  - `config.rs` / `error.rs`: TOML `AppConfig` (`node` and `server` sections) and the error type.

Other files: `grafana-dashboard.json` (dashboard for the exported metrics), `linux/SystemD-service.md` (service install), `Dockerfile`. `prompt.md` is a leftover refactoring prompt, not documentation.

New source files carry the SPDX license header used in existing files.
