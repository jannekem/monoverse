# monoverse
[![Crates.io Version](https://img.shields.io/crates/v/monoverse.svg)](https://crates.io/crates/monoverse)
![Crates.io Total Downloads](https://img.shields.io/crates/d/monoverse)
[![GitHub License](https://img.shields.io/crates/l/monoverse.svg)](https://github.com/jannekem/monoverse/blob/main/LICENSE)

Monoverse is a tool for managing application version numbering using the [CalVer](https://calver.org/) versioning scheme. It is designed to be used with monorepos, but it can also be used with single projects.

The monoverse implementation of CalVer follows the `YY.MM.MICRO` format, where `YY` is the current year, `MM` is the current month, and `MICRO` is a monotonically increasing number that is reset to `0` at the beginning of each month.

Monoverse updates manifest files based on project type and uses git history to decide whether a new release is needed. It can also update dependent files with the new version number so that they stay in sync.

Documentation: https://jannekem.github.io/monoverse/

## Quick start

Create a `monoverse.yaml` file at the repository root, define your projects, then run a release.

```yaml
projects:
  server:
    type: rust
    path: server
  client:
    type: node
    path: client
```

Run the release command:

```bash
monoverse release server --commit --tag
git push && git push --tags
```

This edits the version number (e.g. `25.12.0`) in the server project's manifest file, in this case `Cargo.toml` for a Rust project, creates a commit and a git tag (e.g. `server-25.12.0`). The commit and tag needs to be manually pushed to the remote if desired.

If there are no commits touching the `server` directory since the last update to the line containing the version number, the command is a no-op. A release can be forcibly created using the `--force` flag.

## Configuration

Configuration lives in `monoverse.{yaml,json,toml}` at the repository root. Project types include `rust`, `node`, `helm`, `toml`, `versionfile`, and `yaml`.

See the documentation for full configuration and selector details.

## Installation

### Cargo

Monoverse is written in [Rust](https://www.rust-lang.org/) and requires the Rust toolchain. The easiest way to install Rust is with [rustup](https://rustup.rs/).

Once Rust is installed, you can install the latest published release:

```bash
cargo install monoverse
```

You can also clone the repository and build the binary yourself:

```bash
cargo install --path .
```

### Docker

Monoverse is available as a container image on GitHub Container Registry.

```bash
docker pull ghcr.io/jannekem/monoverse:latest
```

You can run the container against a local repository by mounting it to `/repo`:

```bash
docker run --rm -v /path/to/repository:/repo ghcr.io/jannekem/monoverse:latest release <project-name>
```

The container includes `git`, which makes it easier to integrate `monoverse` into CI/CD pipelines.
