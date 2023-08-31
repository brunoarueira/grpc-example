FROM rust:1.72.0 as build-env

WORKDIR /app
COPY ./protos/* /app/proto/
COPY ./server /app
RUN rustup component add rustfmt && \
    cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build-env /app/target/release/server /
CMD ["./server"]
