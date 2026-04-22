FROM rust:bookworm AS build

# Dependencies
RUN USER=root apt-get update && apt-get upgrade -y

WORKDIR /app
RUN mkdir -p /app/src

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build and install
RUN cargo install --path . --root /usr/local

#####
# Final base
FROM debian:bookworm-slim

# copy the build target
COPY --from=build /usr/local/bin/btcnode-metrics /usr/local/bin/btcnode-metrics

# copy the config
RUN mkdir /etc/btcnode-metrics
COPY ./config.local.toml /etc/btcnode-metrics/config.toml

# expose the Prometheus metrics port specified in the config
EXPOSE 9332

ENV RUST_LOG=info

CMD ["/usr/local/bin/btcnode-metrics", "-c",  "/etc/btcnode-metrics/config.toml"]