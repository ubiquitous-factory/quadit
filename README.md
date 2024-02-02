# quadit

A gitops tool to deploy systemd managed containers on linux. A.K.A quadlets.

## Introduction

`quadit` is focused on managing [quadlets](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html) and running containers in rootless mode using a gitops model.

The `quadit`' core usecase is a `remote edge` scenario enforcing a pure pull model that ensures no inbound access to the device.   

It is written in rust to minimise the overall footprint and improve the power consumption of running a gitops service on low resourced systems.

For more detail on quadlet see [this article](https://www.redhat.com/sysadmin/quadlet-podman). 

[![Build](https://github.com/ubiquitous-factory/quadit/actions/workflows/build.yml/badge.svg)](https://github.com/ubiquitous-factory/quadit/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/quadit.svg)](https://crates.io/crates/quadit)
[![dependency status](https://deps.rs/repo/github/ubiquitous-factory/quadit/status.svg)](https://deps.rs/repo/github/ubiquitous-factory/quadit)

## features

`quadit` is a very opinionated reimplementation of the fantastic [fetchit](https://github.com/containers/fetchit) podman management system. 

Please evaluate the following matrix to understand which one would better suit your needs.

||fetchit|quadit|notes|
|---|---|---|---|
|simple file transfer|<div class="greencircle">&nbsp;</div>|<div class="redcircle"></div>|May be considered as a feature if required|
|ansible|<div class="greencircle"></div>|<div class="redcircle"></div>|Not a quadit goal|
|kube|<div class="greencircle"></div>|<div class="redcircle"></div>|Raw yaml files are not a quadit goal|
|raw|<div class="greencircle"></div>|<div class="redcircle"></div>|Not a quadit goal|
|plain systemd files|<div class="greencircle"></div>|<div class="redcircle"></div>|May be considered as a feature if required|
|inbuilt CLI|<div class="redcircle"></div>|<div class="greencircle"></div>|Main design goal to improve devops experience|
|user quadlet|<div class="redcircle"></div>|<div class="greencircle"></div>|Not available in fetchit [See fetchit issue](https://github.com/containers/fetchit/issues/311)|
|root quadlet|<div class="redcircle"></div>|<div class="redcircle"></div>|May be considered as a feature if required|
|systemd stop|<div class="redcircle"></div>|<div class="greencircle"></div>|[Code exists](https://github.com/containers/fetchit/blob/main/method_containers/systemd/systemd-script#L51) in fetchit but not surfaced in config|
|systemd start|<div class="redcircle"></div>|<div class="greencircle"></div>|Not implemented in `fetchit`|
|auto-update|<div class="greencircle"></div>|<div class="redcircle"></div>|quadit is targeting auto configuration but work is yet to commence|
|.kube|<div class="redcircle"></div>|<div class="yellowcircle"></div>|Standard quadlet file type - Implementation in progress|
|.volume|<div class="redcircle"></div>|<div class="greencircle"></div>|Standard quadlet file type|
|.network|<div class="redcircle"></div>|<div class="greencircle"></div>|Standard quadlet file type|
|.pod|<div class="redcircle"></div>|<div class="greencircle"></div>|Standard quadlet file type|
|.container|<div class="redcircle"></div>|<div class="greencircle"></div>|Standard quadlet file type|


## install

From the edge device running a systemd based distro with the latest podman the following commands:
```
mkdir ~/.quadit
curl -o ~/.quadit https://raw.githubusercontent.com/ubiquitous-factory/quadit/main/samples/config.yaml
curl -o ~/.config/containers/systemd https://raw.githubusercontent.com/ubiquitous-factory/quadit/main/deploy/quadit.container
loginctl enable-linger $USER
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.


### Contributions

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
If you want to contribute to `quadit`, please read our [CONTRIBUTING notes].

[CONTRIBUTING notes]: CONTRIBUTING.md


<style>
    .redcircle {
     width: 10px; 
            height: 10px; 
            background-color: red; 
            border-radius: 50%; 
            margin: auto;
            } 
    .greencircle {
width: 10px; 
            height: 10px; 
            background-color: green; 
            border-radius: 50%; 
            margin: auto;
    } 
    .yellowcircle {
width: 10px; 
            height: 10px; 
            background-color: yellow; 
            border-radius: 50%; 
            margin: auto;
    } 
    </style>