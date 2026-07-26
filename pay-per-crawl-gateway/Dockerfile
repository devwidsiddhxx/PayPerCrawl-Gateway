FROM rust:1.80-slim as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin pay-per-crawl-gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/pay-per-crawl-gateway /usr/local/bin/
CMD ["pay-per-crawl-gateway"]
