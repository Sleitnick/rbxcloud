# Install

## Install CLI

There are a few different ways to install the `rbxcloud` CLI.

### [Aftman](https://github.com/LPGhatguy/aftman) <small>(Preferred)</small>
Run the `aftman add` command within your project directory. This will add `rbxcloud` to the project's `aftman.toml` file (or create one if it doesn't yet exist).
```sh
$ aftman add Sleitnick/rbxcloud@0.16.0
```

Next, run `aftman install` to install `rbxcloud`.

### [Foreman](https://github.com/Roblox/foreman)
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file.
```toml
# foreman.toml
[tools]
rbxcloud = { github = "Sleitnick/rbxcloud", version = "0.16.0" }
```

Next, run `foreman install` to install `rbxcloud`.

??? info "Legacy"
	Aftman is preferred over Foreman. For more information, see the [Aftman](https://github.com/LPGhatguy/aftman) GitHub repository.

### [Releases](https://github.com/Sleitnick/rbxcloud/releases)
A few prebuilt binaries are compiled and attached to each release.

1. Go to the [`releases`](https://github.com/Sleitnick/rbxcloud/releases) page
1. Navigate to the desired version (e.g. the [latest version](https://github.com/Sleitnick/rbxcloud/releases/latest))
1. Download and unzip the desired executable
1. Relocate the executable to a more permanent place
1. Change your OS path variable to point to the executable

## Verify
Run the tool with the `--version` flag to verify the installation succeeded.
```sh
$ rbxcloud --version
```
