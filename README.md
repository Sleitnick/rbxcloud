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

## Install
### Foreman
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file:
```toml
rbxcloud = { github = "Sleitnick/rbxcloud", version = "0.2.0" }
```

### Cargo
Install from cargo:
```sh
$ cargo install rbxcloud@0.2.0
```

### From Release
Download and unzip the tool for your OS from the [releases](https://github.com/Sleitnick/rbxcloud/releases) page. You will then need to put it in your desired location and point your path variable to the executable location.

### From Source
Download the repository and build from the source:
```sh
# Clone repo and set as current directory:
$ git clone https://github.com/Sleitnick/rbxcloud.git
$ cd rbxcloud

# Build binary:
$ cargo build --release
```

## Documentation
Visit the documentation site for information on installation and usage: https://sleitnick.github.io/rbxcloud/
