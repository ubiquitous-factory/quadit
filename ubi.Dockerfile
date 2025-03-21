FROM docker.io/fedora@sha256:d0207dbb078ee261852590b9a8f1ab1f8320547be79a2f39af9f3d23db33735e as build

RUN dnf install -y gcc openssl-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

COPY . /app-build

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

RUN cargo build --release

FROM docker.io/fedora@sha256:d0207dbb078ee261852590b9a8f1ab1f8320547be79a2f39af9f3d23db33735e

RUN  dnf install -y gcc openssl-devel

WORKDIR "/app"
COPY --from=rhel9builder /app-build/target/release/quadit ./

CMD [ "./quadit" ]
