FROM alpine:3.23.4@sha256:5b10f432ef3da1b8d4c7eb6c487f2f5a8f096bc91145e68878dd4a5019afde11

ARG TARGET
COPY ./target/${TARGET}/release/consistent_whitespace /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["consistent_whitespace"]
