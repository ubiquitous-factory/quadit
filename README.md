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

`quadit` is an opinionated reimplementation of the fantastic [fetchit](https://github.com/containers/fetchit) podman management system. 

Please evaluate the following matrix to understand which one would better suit your needs.

||fetchit|quadit|notes|
|---|---|---|---|
|simple file transfer|yes|no|May be considered as a feature if required|
|ansible|yes|no|Not a quadit goal|
|raw|yes|no|Not a quadit goal|
|plain systemd files|yes|no|May be considered as a feature if required|
|user quadlet|no|yes|Not available in fetchit [See fetchit issue](https://github.com/containers/fetchit/issues/311)|
|root quadlet|no|no|May be considered as a feature if required|
|systemd stop|no|yes|[Code exists](https://github.com/containers/fetchit/blob/main/method_containers/systemd/systemd-script#L51) in fetchit but not surfaced in config|
|systemd start|no|yes|Not implemented in `fetchit` at time of writing|
|auto-update|yes|no|quadit is targeting auto configuration but work is yet to commence|
|.kube|no|yes|Standard quadlet file type|
|.volume|no|yes|Standard quadlet file type|
|.network|no|yes|Standard quadlet file type|
|.pod|no|yes|Standard quadlet file type|
|.container|no|yes|Standard quadlet file type|

## install

From the edge device running a systemd based distro with the latest podman the following commands:
```
sudo setsebool -P container_manage_cgroup true
mkdir ~/.quadit
curl -o ~/.quadit/config.yaml https://raw.githubusercontent.com/ubiquitous-factory/quadit/main/samples/config.yaml
mkdir -p ~/.config/containers/systemd
curl -o ~/.config/containers/systemd/quadit.container https://raw.githubusercontent.com/ubiquitous-factory/quadit/main/deploy/quadit.container
loginctl enable-linger $USER
systemctl --user daemon-reload
systemctl --user start quadit
```

### environment variables
None of these environment variables should need tweaking but the options are documented as they are available.

|Name|Required|Default|Description|
|---|---|---|
|
|BOOT_URL|NO|<Empty>|Bootstrap the service from remote `config.yaml` hosted at a url. Overrides the local `config.yaml`| 
|LOCAL|NO|`no`|If set to a 'yes' then the exe will assume it's not in a container and run with the local users configuration for `$HOME` and not use `/opt` locations| 
|LOG_LEVEL|NO|`info`| Can be `error`, `warn`, `info`, `debug`, `trace`|
|JOB_PATH|NO|<Empty>|Left empty for testing but in deployment it's set to `/tmp` in the `quadit.container` file|
|JOB_FOLDER|NO|`jobs`|The name of the folder to save jobs.|
|PODMAN_UNIT_PATH|NO|`$HOME/.config/containers/systemd`|The location where the container files should be written on the host machine|
|HOME|NO|%u|Set by systemd parameter `%u` but can be overridden in the `quadit.container` file|
|XDG_RUNTIME_DIR|NO|`/run/user/%U`|Used by systemd to find a user-specific directory in which it can store small temporary files. Sometimes not available in headless environments
|PODMAN_SYSTEMD_UNIT|NO|%n|Set by systemd - the name of the unit|

|SYSTEMCTL_PATH|NO|`/usr/bin/systemctl`|Path to the `systemctl` binary|

## Support

This project is kindly sponsored by [Mehal Technologies](https://mehal.tech) and they offer commercial support for this software as well as providing cloud services to deliver advanced GitOps use cases. 

The project follows latest in all downstream dependencies. 

We won't intentionally break compatibility on new versions and we are happy to receive changes from the community to provide backwards compatibility but the project will prioritize new capabilities over backwards compatibility where a decision needs to be made.  

The current list of downstream OS service dependencies is:  
* podman >= 5.4.0
* fedora >= 41
* Red Hat Enterprise Linux >= 9
* ubuntu >= 22.04

**N.B This list is updated per release**

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
