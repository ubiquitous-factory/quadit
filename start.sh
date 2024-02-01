#! /bin/bash
podman run -it --privileged --security-opt label=disable -e XDG_RUNTIME_DIR=/run/user/1000 -e JOB_PATH=/tmp --init-path=/usr/sbin/init --userns=keep-id --pid=host --mount type=tmpfs,destination=/run/user/1000,rw=true --mount type=bind,source=/sys/fs/cgroup,destination=/sys/fs/cgroup,ro=true --mount type=bind,source=/run/user/1000/systemd,destination=/run/user/1000/systemd,rw=true -v quadit-volume:/opt -v /home/anton/.quadit:/opt/config:rw -v /home/anton/.config/containers/systemd:/opt/containers:rw -v/run/user/1000/podman/podman.sock:/run/podman/podman.sock --userns keep-id quay.io/ubiquitous-factory/quadit:v0.1.2

# podman run -it --privileged --security-opt label=disable -e XDG_RUNTIME_DIR=/run/user/1000 -e JOB_PATH=/tmp --init-path=/usr/sbin/init --userns=keep-id --pid=host --mount type=tmpfs,destination=/run/user/1000,rw=true --mount type=bind,source=/sys/fs/cgroup,destination=/sys/fs/cgroup,ro=true --mount type=bind,source=/run/user/1000/systemd,destination=/run/user/1000/systemd,rw=true registry.access.redhat.com/ubi8/ubi:latest /bin/bash 