FROM alpine:3.22.0@sha256:8a1f59ffb675680d47db6337b49d22281a139e9d709335b492be023728e11715

COPY ./target/x86_64-unknown-linux-musl/release/consistent_whitespace /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["consistent_whitespace"]
