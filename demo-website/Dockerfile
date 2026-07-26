FROM rust:1.80-slim as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin demo-website

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/demo-website /usr/local/bin/
CMD ["demo-website"]
