FROM registry.access.redhat.com/ubi9/ubi as rhel9builder

RUN yum install -y gcc openssl-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

COPY . /app-build

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

RUN cargo build --release

FROM registry.access.redhat.com/ubi9/ubi-minimal

RUN  microdnf update && microdnf install -y procps-ng

WORKDIR "/app"
COPY --from=rhel9builder /app-build/target/release/quadit ./

CMD [ "./quadit" ]
