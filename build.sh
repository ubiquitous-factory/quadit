#! /bin/bash

tag=v0.1.4
img=quay.io/ubiquitous-factory/quadit:$tag

podman build --platform linux/amd64 -t $img-linux-amd64 .
podman build --platform linux/arm64/v8 -t $img-linux-arm64 .

podman push $img-linux-amd64 
podman push $img-linux-arm64 

podman manifest create $img $img-linux-amd64 $img-linux-arm64

podman manifest push $img