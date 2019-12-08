FROM ekidd/rust-musl-builder:nightly-2019-11-06 as builder

RUN USER=root cargo new --bin rust_tide_backend
WORKDIR ./rust_tide_backend
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/rust_tide_backend*
RUN cargo build --release


FROM alpine:latest

EXPOSE 8080

ENV TZ=Etc/UTC \
    TPS_USER=tps

RUN addgroup -S $TPS_USER \
    && adduser -S -g $TPS_USER $TPS_USER

RUN apk update \
    && apk add --no-cache add ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/rust_tide_backend/target/x86_64-unknown-linux-musl/release/rust-tide-backend /usr/local/bin/rust-tide-backend
COPY --from=builder /home/rust/src/rust_tide_backend/config /usr/local/bin/config
COPY --from=builder /home/rust/src/rust_tide_backend/demo-data /usr/local/bin/demo-data

WORKDIR /usr/local/bin

USER $TPS_USER

CMD ["./rust-tide-backend"]
