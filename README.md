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
|simple file transfer|:green_circle:|:red_circle:|May be considered as a feature if required|
|ansible|:green_circle:|:red_circle:|Not a quadit goal|
|kube|:green_circle:|:red_circle:|Not a quadit goal|
|raw|:green_circle:|:red_circle:|Not a quadit goal|
|root systemd|:green_circle:|:red_circle:|Not a quadit goal|
|user systemd|:green_circle:|:red_circle:|Not a quadit goal|
|auto-update|:green_circle:|:green_circle:|quadit is targeting auto configuration|
|root quadlet|:red_circle:|:red_circle:|Currently not a quadit goal but open to re-evaluation|
|user quadlet|:red_circle:|:green_circle:|Not available in fetchit [See fetchit issue](https://github.com/containers/fetchit/issues/311)|
|systemd stop|:red_circle:|:green_circle:|[Code exists](https://github.com/containers/fetchit/blob/main/method_containers/systemd/systemd-script#L51) in fetchit but not surfaced in config|
|systemd start|:red_circle:|:green_circle:|Not implemented in `fetchit`|



## install

### development 

```
cargo install quadit
quadit config
quadit deploy
```

### production -TBD

```
loginctl enable-linger <username>
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
