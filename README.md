# Roblox Open Cloud CLI &emsp; ![Logo](imgs/icon_32_bottom.png)
[![CI](https://github.com/Sleitnick/rbxcloud/actions/workflows/ci.yaml/badge.svg)](https://github.com/Sleitnick/rbxcloud/actions/workflows/ci.yaml)
[![Release](https://github.com/Sleitnick/rbxcloud/actions/workflows/release.yaml/badge.svg)](https://github.com/Sleitnick/rbxcloud/actions/workflows/release.yaml)
[![Crate](https://img.shields.io/crates/v/rbxcloud.svg)](https://crates.io/crates/rbxcloud)
[![Docs](https://docs.rs/rbxcloud/badge.svg)](https://docs.rs/rbxcloud)

The `rbxcloud` CLI lets developers easily communicate with the Roblox Open Cloud APIs. The underlying library can also be used to build tools and applications that utilize the various APIs.

Possible use-cases:
- Deployment pipelines
- Live-ops
- Custom analytics
- Handling data removal requests (see [GDPR & CCPA info](https://create.roblox.com/docs/production/publishing/about-GDPR-and-CCPA))
- Debugging DataStores

## Install CLI
### Aftman
Run the `aftman add` command within your project directory. This will add `rbxcloud` to the project's `aftman.toml` file (or create one if it doesn't yet exist).
```sh
$ aftman add Sleitnick/rbxcloud@0.2.1
```

### From Release
Download and unzip the tool for your OS from the [releases](https://github.com/Sleitnick/rbxcloud/releases) page. You will then need to put it in your desired location and point your path variable to the executable location.

## CLI Documentation
Visit the documentation site for information on installation and usage: https://sleitnick.github.io/rbxcloud/

## Install Rust Library
### Add Dependency

To use `rbxcloud` in a Rust project, simply add `rbxcloud` to the `Cargo.toml` dependency list.
```toml
[dependencies]
rbxcloud = "0.2.1"
```

Alternatively, use `cargo add`.
```sh
$ cargo add rbxcloud
```

See the [docs.rs](https://docs.rs/rbxcloud/latest/rbxcloud/) page for library documentation.
