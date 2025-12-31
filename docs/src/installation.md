# Installation

## Cargo

Monoverse is written in [Rust](https://www.rust-lang.org/) and requires the Rust toolchain. The easiest way to install Rust is with [rustup](https://rustup.rs/).

Once Rust is installed, you can install the latest published release:

```bash
cargo install monoverse
```

You can also clone the repository and build the binary yourself:

```bash
cargo install --path .
```

## Docker

Monoverse is available as a container image on GitHub Container Registry.

```bash
docker pull ghcr.io/jannekem/monoverse:latest
```

You can run the container against a local repository by mounting it to `/repo`:

```bash
docker run --rm -v /path/to/repository:/repo ghcr.io/jannekem/monoverse:latest release <project-name>
```

The container includes `git`, which makes it easier to integrate `monoverse` into CI/CD pipelines.
