FROM rust:latest as builder
WORKDIR /app
COPY . .
ENV CARGO_REGISTRY=https://rsproxy.cn/crates.io-index
RUN cargo build --release \
    && strip target/release/get_address_by_ip \
    && rm -rf target/debug

FROM messense/rust-musl-cross:x86_64-musl
WORKDIR /app
COPY --from=builder /app/target/release/get_address_by_ip .
EXPOSE 8080
CMD ./get_address_by_ip /app/