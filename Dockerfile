FROM rust:1.57.0-alpine

RUN apk add --no-cache musl-dev
WORKDIR /opt/rust-warp-docker
COPY . ./
RUN cargo build --release

EXPOSE 8080
CMD ["/opt/rust-warp-docker/target/release/rust-backend"]