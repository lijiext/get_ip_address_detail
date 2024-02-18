FROM rust:latest as builder
WORKDIR /app
COPY . .
ENV CARGO_REGISTRY=https://rsproxy.cn/crates.io-index
RUN cargo build --release \
    && strip target/release/get_address_by_ip \
    && rm -rf target/debug

FROM redhat/ubi8-micro:latest
RUN RUN yum -y install httpd; yum clean all;
WORKDIR /app
COPY --from=builder /app/target/release/get_address_by_ip .
EXPOSE 8080
CMD ./get_address_by_ip /app/