FROM cgr.dev/chainguard/rust:latest-dev AS build

USER root
RUN apk update && apk add openssl-dev

WORKDIR /app

COPY ./src ./src
COPY Cargo.toml  ./
RUN cargo build --release

FROM cgr.dev/chainguard/glibc-dynamic
COPY --from=build /usr/lib/libssl.so.3 /usr/lib/libssl.so.3
COPY --from=build /usr/lib/libcrypto.so.3 /usr/lib/libcrypto.so.3 
COPY --from=build --chown=nonroot:nonroot /app/target/release/quadit /usr/local/bin/quadit
CMD [ "/usr/local/bin/quadit" ]
