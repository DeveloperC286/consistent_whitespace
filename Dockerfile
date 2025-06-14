FROM alpine:3.22.0

RUN apk add --no-cache \
    git=2.49.0-r0

COPY ./target/x86_64-unknown-linux-musl/release/consistent_whitespace /usr/local/bin/

ENTRYPOINT ["consistent_whitespace"]
