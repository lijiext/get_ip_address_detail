FROM rust:latest as builder
WORKDIR /app
COPY . .
ENV CARGO_REGISTRY=https://rsproxy.cn/crates.io-index
RUN cargo build --release \
    && strip target/release/get_address_by_ip \
    && rm -rf target/debug

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev
WORKDIR /app
COPY --from=builder /app/target/release/get_address_by_ip .
EXPOSE 8080
CMD ./get_address_by_ip /app/