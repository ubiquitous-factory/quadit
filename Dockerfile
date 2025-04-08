FROM docker.io/fedora@sha256:f84a7b765ce09163d11de44452a4b56c1b2f5571b6f640b3b973c6afc4e63212 as build

RUN dnf install -y gcc openssl-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

COPY ./src ./src
COPY Cargo.toml  ./
RUN cargo build --release

FROM docker.io/fedora@sha256:f84a7b765ce09163d11de44452a4b56c1b2f5571b6f640b3b973c6afc4e63212

ENV container docker
RUN dnf -y update; dnf clean all
RUN dnf -y install systemd openssl-devel; dnf clean all; \
    (cd /lib/systemd/system/sysinit.target.wants/; for i in *; do [ $i == systemd-tmpfiles-setup.service ] || rm -f $i; done); \
    rm -f /lib/systemd/system/multi-user.target.wants/*; \
    rm -f /etc/systemd/system/*.wants/*; \
    rm -f /lib/systemd/system/local-fs.target.wants/*; \
    rm -f /lib/systemd/system/sockets.target.wants/*udev*; \
    rm -f /lib/systemd/system/sockets.target.wants/*initctl*; \
    rm -f /lib/systemd/system/basic.target.wants/*; \
    rm -f /lib/systemd/system/anaconda.target.wants/*;
VOLUME [ "/sys/fs/cgroup" ]

WORKDIR "/app"
COPY --from=build /app-build/target/release/quadit ./

CMD [ "./quadit" ]
