FROM alpine:3.24.4@sha256:f59abe656b692d901e184be3445e084786602108599d2a51d229c0f5a48c71bf

ARG TARGET
COPY ./target/${TARGET}/release/consistent_whitespace /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["consistent_whitespace"]
