# Contributing
The `rbxcloud` project is open-source and welcomes PRs. If you wish to contribute to the project, please follow the guidelines listed here.

## Purpose
The goal of the `rbxcloud` project is to provide an API and CLI tool to access Roblox's [Open Cloud](https://create.roblox.com/docs/cloud/open-cloud). As Roblox's Open Cloud matures and adds new features, the `rbxcloud` project should attempt to mirror these additions.

## Structure
The `rbxcloud` project is split into two main chunks:
1. Rust library for accessing Roblox Open Cloud
2. CLI tool for accessing the Roblox Open Cloud

This separation allows `rbxcloud` to be used both as a standalone CLI tool and as a library for accessing Roblox Open Cloud.

### Rust Library
The Rust library (`src/rbx`) implements both a "low" and "high" level access into Roblox Open Cloud.

The low-level access implementations are internal to the project, and are designed to wrap around the actual HTTP Rest endpoints that Roblox provides. These Rust function APIs are stateless and are meant to provide basic error handling around the web endpoints.

The high-level access implementations are named "clients" and wrap around the low-level implementations. In Rust, these are structs that represent a category of endpoints (e.g. the MessagingService endpoints). These client structs are the publicly-facing APIs that end-users should be interacting with in Rust code.

### CLI
The CLI portion (`src/cli`) of the project allows users to access the Roblox Open Cloud from the command line.

Internally, the CLI implementation wraps around the high-level client implementations within the `rbx` library portion of the project. The CLI implementations should _not_ touch the lower-level function APIs. In other words, any CLI code should be interacting with client structs.

CLI processing is all handled by the [`clap`](https://docs.rs/clap/latest/clap/) crate.

## Opening PRs
Any PR should be accommodated by an associated GitHub Issue. A PR that does not have an associated GitHub Issue will be closed.

If at all possible, do not introduce any new dependencies into the project. Similarly, any changes to the versions of current dependencies must give justification for the version changes.

## Bug Fix PRs
When opening up a PR for a bug fix, please ensure the following:
- The associated GitHub Issue explains the bug in detail.
- The PR fixes said bug.
- The PR does not introduce new features.
- The PR explains what the new code does in order to fix the problem.
- The new code has been tested and successfully resolves the bug in question.
- Please try to not introduce any new dependencies.

## New Feature PRs
When opening up a PR for a new feature, please ensure the following:
- The associated GitHub Issue explains the new feature in detail.
- The new feature relates directly to mirroring a Roblox Open Cloud feature that is not currently supported.
- The PR does not introduce features beyond the associated GitHub Issue.
- The new code has been tested and successfully implements the new feature in question.
- Please try to not introduce any new dependencies.

## New Feature Expectations
When adding a new feature to support a new Roblox Open Cloud endpoint, please ensure that the code fits within the same structure as other features:

- The "low-level" implementation should be present within the `src/rbx/vX` directory, where "X" represents the Roblox Open Cloud resource version.
- The "high-level" client implementation should be present within the `src/rbx/mod.rs` source file. This is the main API for user code, as well as the CLI.
- The CLI implementation should be present within the `src/cli` directory, and wired up within the `src/cli/mod.rs` source file.
- Documentation has been added to the `docs/cli` directory.
- The documentation `mkdocs.yml` configuration file has been updated to include any new documentation files.

As an example, take a look at the implementation for the Universe endpoints:
- Low-level implementation: `src/rbx/v2/universe.rs`.
- High-level implementation: `src/rbx/v2/mod.rs` (search for "UniverseClient" and look at the struct and the implementation).
- CLI implementation: `src/cli/universe_cli.rs`
- CLI integration: `src/cli/mod.rs` (search for "Universe" and see its usage within the `Command` enum and the `Cli` implementation).
