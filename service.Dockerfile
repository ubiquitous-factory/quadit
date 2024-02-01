FROM registry.access.redhat.com/ubi8/ubi as builder

RUN yum install -y gcc openssl-devel procps-ng && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

COPY Cargo.toml /app-build/Cargo.toml
COPY src /app-build/src
WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

RUN cargo build --release --no-default-features

FROM registry.access.redhat.com/ubi8/ubi

WORKDIR "/app"
COPY --from=builder /app-build/target/release/quadit ./

CMD [ "./quadit" ]
