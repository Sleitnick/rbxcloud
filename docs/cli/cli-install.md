# Install

## Install CLI

There are a few different ways to install the `rbxcloud` CLI.

### Foreman
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file.
```toml
rbxcloud = { github = "Sleitnick/rbxcloud", version = "0.1.0-alpha.7" }
```

Next, run `foreman install`. To update, simply change the `version` field and then run `foreman install` again.

### Cargo
Install or update the `rbxcloud` package using `cargo install`.
```sh
cargo install rbxcloud
```

Alternatively, install or update to a specific version.
```sh
cargo install rbxcloud@0.1.0-alpha.7
```

### Releases
A few prebuilt binaries are compiled and attached to each release. These are primarily used for Foreman, but can be downloaded directly.

1. Go to the [`releases`](https://github.com/Sleitnick/rbxcloud/releases) page
1. Navigate to the desired version (e.g. the [latest version](https://github.com/Sleitnick/rbxcloud/releases/latest))
1. Download and unzip the desired executable
1. Relocate the executable to a more permanent place
1. Change your OS path variable to point to the executable

### Source
The `rbxcloud` binary can easily be built from the source. Clone the repository and build the project with `cargo`.
```sh
# Clone repo and set as current directory:
$ git clone https://github.com/Sleitnick/rbxcloud.git
$ cd rbxcloud

# Build binary:
$ cargo build --release
```

## Test
Run the tool with the `--version` flag to verify the installation succeeded.
```sh
$ rbxcloud --version
```
