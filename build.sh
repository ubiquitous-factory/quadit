#! /bin/bash

tag=v0.1.0
img=quay.io/ubiquitous-factory/quadit:$tag
podman manifest create -a $img

podman build --platform linux/amd64,linux/arm64  -f service.Dockerfile --manifest $img  .

podman manifest push $img