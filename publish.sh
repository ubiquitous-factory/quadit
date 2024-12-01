#! /bin/bash 

img=quay.io/mehal_tech/quadit
podman manifest create $img

# Build the image attaching them to the manifest
podman build --platform linux/amd64,linux/arm64  --manifest $img .

# Finally publish the manifest
podman manifest push $img