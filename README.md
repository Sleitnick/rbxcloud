# Roblox Open Cloud CLI &emsp; ![Logo](imgs/icon_32_bottom.png)
[![CI](https://github.com/Sleitnick/rbxcloud/actions/workflows/ci.yaml/badge.svg)](https://github.com/Sleitnick/rbxcloud/actions/workflows/ci.yaml)
[![Release](https://github.com/Sleitnick/rbxcloud/actions/workflows/release.yaml/badge.svg)](https://github.com/Sleitnick/rbxcloud/actions/workflows/release.yaml)

:warning: Under Development :warning:

The `rbxcloud` CLI lets developers easily communicate with the Roblox Open Cloud APIs.

Common use-cases:
- CD pipelines
- Debugging DataStores
- _TODO: More use-cases_

## Install
### Foreman
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file:
```toml
rbxcloud = { github = "Sleitnick/rbxcloud", version = "0.1.0-alpha.9" }
```

### Cargo
Install from cargo:
```sh
$ cargo install rbxcloud@0.1.0-alpha.9
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
