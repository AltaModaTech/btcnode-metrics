FROM rust:bookworm AS build

# Dependencies
RUN USER=root apt-get update && apt-get upgrade -y

# Install the lastest version directly into path
RUN cargo install btcnode-metrics --root /usr/local

#####
# Final base
FROM debian:bookworm-slim

COPY --from=build /usr/local/bin/btcnode-metrics /usr/local/bin/btcnode-metrics

# copy the bin and config
RUN mkdir /etc/btcnode-metrics
COPY ./config.local.toml /etc/btcnode-metrics/config.toml

# expose the port identified in the config
EXPOSE 9898

ENV RUST_LOG=info

CMD ["/usr/local/bin/btcnode-metrics", "-c",  "/etc/btcnode-metrics/config.toml"]