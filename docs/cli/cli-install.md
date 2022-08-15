# Install

## Install CLI

There are a few different ways to install the `rbxcloud` CLI.

### [Aftman](https://github.com/LPGhatguy/aftman)
Run the `aftman add` command within your project directory. This will add `rbxcloud` to the project's `aftman.toml` file (or create one if it doesn't yet exist).
```sh
$ aftman add Sleitnick/rbxcloud@0.2.0
```

### Foreman
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file.
```toml
# foreman.toml
[tools]
rbxcloud = { github = "Sleitnick/rbxcloud", version = "0.2.0" }
```

??? info "Legacy"
	Aftman is preferred over Foreman. For more information, see the [Aftman](https://github.com/LPGhatguy/aftman) GitHub repository.

### Releases
A few prebuilt binaries are compiled and attached to each release. These are primarily used for Foreman, but can be downloaded directly.

1. Go to the [`releases`](https://github.com/Sleitnick/rbxcloud/releases) page
1. Navigate to the desired version (e.g. the [latest version](https://github.com/Sleitnick/rbxcloud/releases/latest))
1. Download and unzip the desired executable
1. Relocate the executable to a more permanent place
1. Change your OS path variable to point to the executable

Next, run `foreman install`. To update, simply change the `version` field and then run `foreman install` again.

## Verify
Run the tool with the `--version` flag to verify the installation succeeded.
```sh
$ rbxcloud --version
```
