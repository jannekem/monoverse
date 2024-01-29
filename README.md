# monoverse

- [Introduction](#introduction)
- [Installation](#installation)
- [Configuration](#configuration)
  - [Projects](#projects)
  - [Example YAML configuration](#example-yaml-configuration)
  - [Example TOML configuration](#example-toml-configuration)
- [Usage](#usage)

_This project is still in its early stages and as such it is not yet ready for production use._

## Introduction

Monoverse is a tool for managing application version numbering using the [CalVer](https://calver.org/) versioning scheme. It is designed to be used with monorepos, but it can also be used with single projects as witnessed by the `monoverse` project itself.

The reality for many projects is that they don't actually need to follow [semantic versioning](https://semver.org/). For example, if you are building a web application with lots of microservices, you're generally not going to be publishing them as libraries. Instead, they'll be deployed as individual APIs that each have their own lifecycle.

CalVer is a simple versioning scheme that is based on the calendar. The `monoverse` implementation of CalVer follows the `YY.MM.MICRO` format, where `YY` is the current year, `MM` is the current month, and `MICRO` is a monotonically increasing number that is reset to `0` at the beginning of each month.

[Ubuntu](https://wiki.ubuntu.com/Releases) is a famous example of a project that uses (a variation of) CalVer. Just remember that it is not a replacement for semantic versioning. If you're building a library, you should probably stick to semantic versioning as going back from CalVer is not easy.

## Installation

Monoverse is written in [Rust](https://www.rust-lang.org/) and as such it currently requires the Rust toolchain to be installed. The easiest way to install Rust is by using [rustup](https://rustup.rs/).

Once Rust is installed and you have cloned the repository, you can install monoverse by running the following command from the root of the repository:

```bash
cargo install --path .
```

This will build the `monoverse` binary and install it in your `~/.cargo/bin` directory, after which it can be used from anywhere.

## Configuration

Monoverse can be configured by defining a configuration file at the root of the project. The supported file formats are `json`, `yaml` and `toml`. The configuration file must be named `monoverse.{json,yaml,toml}`.

The configuration file defines the projects that are part of the monorepo. Each project is defined by a type and a path.

### Projects

Applications are defined in the `projects` section of the configuration file.

Each project is represented by a key-value pair, where the key is the name of the project and the value is a map with the following keys:

| Key             | Description                                   | Allowed values                                                                                                                |
| --------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `type`          | The type of the project.                      | `rust`, `node`, `helm`                                                                                                        |
| `path`          | The path to the project.                      | Any valid directory path relative to the repository root. If omitted, the repository root is used instead.                    |
| `manifest_path` | The path to the manifest file of the project. | Any valid file path relative to the project root. If omitted, the manifest file is assumed to be located at the project path. |

### Example YAML configuration

```yaml
projects:
  server:
    type: rust
    path: server
  client:
    type: node
    path: client
  nginx:
    type: helm
    path: apps/nginx
    manifest_path: apps/nginx/deployment/Chart.yaml
```

### Example TOML configuration

```toml
[projects.server]
type = "rust"
path = "server"

[projects.client]
type = "node"
path = "client"

[projects.nginx]
type = "helm"
path = "apps/nginx"
manifest_path = "apps/nginx/deployment/Chart.yaml"
```

## Usage

After installing monoverse and creating a configuration file, you can use it to manage the version numbers of your projects.

You can create a new version by running the following command:

```bash
monoverse release <project>
```

where `<project>` is the key of the project as defined in the configuration file.

Monoverse will then check if the project has been modified since the last release. If there are changes, it will craft a new version number and update the project's manifest file depending on its type.
