FROM registry.access.redhat.com/ubi8/ubi as rhel8builder

RUN yum install -y gcc openssl-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

COPY . /app-build

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

RUN cargo build --release --no-default-features

FROM registry.access.redhat.com/ubi8/ubi-minimal

RUN  microdnf update && microdnf install -y procps-ng

WORKDIR "/app"
COPY --from=rhel8builder /app-build/target/release/quadit ./

CMD [ "./quadit" ]
