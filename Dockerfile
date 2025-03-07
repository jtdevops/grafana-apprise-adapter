FROM rust:1.75-slim AS builder

WORKDIR /usr/src/grafana-apprise-adapter

COPY ./src /usr/src/grafana-apprise-adapter/src
COPY Cargo.toml /usr/src/grafana-apprise-adapter/Cargo.toml
COPY Cargo.lock /usr/src/grafana-apprise-adapter/Cargo.lock

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/grafana-apprise-adapter/target/release/grafana-apprise-adapter /usr/local/bin/grafana-apprise-adapter

EXPOSE 5000

CMD ["/usr/local/bin/grafana-apprise-adapter"]
