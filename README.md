# monoverse
[![Crates.io Version](https://img.shields.io/crates/v/monoverse.svg)](https://crates.io/crates/monoverse)
![Crates.io Total Downloads](https://img.shields.io/crates/d/monoverse)
[![GitHub License](https://img.shields.io/crates/l/monoverse.svg)](https://github.com/jannekem/monoverse/blob/main/LICENSE)

- [Introduction](#introduction)
- [Installation](#installation)
- [Configuration](#configuration)
  - [Projects](#projects)
  - [Project dependents](#project-dependents)
  - [Example YAML configuration](#example-yaml-configuration)
  - [Example TOML configuration](#example-toml-configuration)
- [Usage](#usage)
  - [Release](#release)
  - [Next](#next)

_This project is still in its early stages and as such it is not yet ready for production use._

## Introduction

Monoverse is a tool for managing application version numbering using the [CalVer](https://calver.org/) versioning scheme. It is designed to be used with monorepos, but it can also be used with single projects.

The reality for many projects is that they don't actually need to follow [semantic versioning](https://semver.org/). For example, if you are building a web application with lots of microservices, you're generally not going to be publishing them as libraries. Instead, they'll be deployed as individual APIs that each have their own lifecycle.

CalVer is a simple versioning scheme that is based on the calendar. The `monoverse` implementation of CalVer follows the `YY.MM.MICRO` format, where `YY` is the current year, `MM` is the current month, and `MICRO` is a monotonically increasing number that is reset to `0` at the beginning of each month.

[Ubuntu](https://wiki.ubuntu.com/Releases) is a famous example of a project that uses (a variation of) CalVer. Just remember that it is not a replacement for semantic versioning. If you're building a library, you should probably stick to semantic versioning as going back from CalVer is not easy.

## Installation

Monoverse is written in [Rust](https://www.rust-lang.org/) and as such it currently requires the Rust toolchain to be installed. The easiest way to install Rust is by using [rustup](https://rustup.rs/).

Once you have Rust installed, you can install the latest published release of `monoverse` by running the following command:

```bash
cargo install monoverse
```

This will build the `monoverse` binary and install it in your `~/.cargo/bin` directory, after which it can be used from anywhere.

You can also clone the repository and build the binary yourself with the following command:

```bash
cargo install --path .
```

## Configuration

Monoverse can be configured by defining a configuration file at the root of the project. The supported file formats are `json`, `yaml` and `toml`. The configuration file must be named `monoverse.{json,yaml,toml}`.

The configuration file defines the projects that are part of the monorepo. Each project is defined by a type and a path.

### Projects

Applications are defined in the `projects` section of the configuration file.

Each project is represented by a key-value pair, where the key is the name of the project and the value is a map with the following keys:

| Key             | Description                                   | Allowed values                                                                                                                                                   |
| --------------- | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | The type of the project.                      | `rust`, `node`, `helm`, `toml`, `yaml`                                                                                                                           |
| `path`          | The path to the project.                      | Any valid directory path relative to the repository root. If omitted, the repository root is used instead.                                                       |
| `manifest_path` | The path to the manifest file of the project. | Any valid file path relative to the project root. If omitted, the manifest file is assumed to be located at the project path.                                    |
| `selector`      | The selector for the version number.          | The format of the selector depends on the `type` of the project.                                                                                                 |
| `dependents`    | The dependents of the project.                | A list of dependent files which should be updated when the project is released. For more information, see the [Project dependents](#project-dependents) section. |

Selector formats for project types that use the `selector` key:

| Project type | Selector format                                                                           |
| ------------ | ----------------------------------------------------------------------------------------- |
| `toml`       | Dot-separated path to the version number in the TOML file. For example: `package.version` |
| `yaml`       | Dot-separated path to the version number in the YAML file. For example: `package.version` |


### Project dependents

Projects can also have dependents. This is useful when you have a project that is used by other projects or files in the repository.

When a project is released, its dependents will be updated with the new version number. Dependents are defined in the `dependents` section of the project configuration.

Dependent settings can contain the following keys:

| Key        | Description                                                | Allowed values                                                                                                              |
| ---------- | ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `type`     | The type of the dependent.                                 | `regex`, `toml`                                                                                                             |
| `path`     | The path to the dependent file.                            | Any valid file path relative to the project root.                                                                           |
| `selector` | The selector for the version number in the dependent file. | A selector for the version number in the dependent file. The format of the selector depends on the `type` of the dependent. |
| `replace`  | String to replace the selector match with.                 | A format string to replace the selector match with. The format of the string depends on the `type` of the dependent.        |

Selector formats for different dependent types:

| Dependent type | Selector format                                                                                                                                                                                                 |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`        | A regular expression that matches any text in the dependent file. Note that you will need to escape characters that would otherwise be interpreted by the YAML/TOML parsing. For example: `v\\d+\\.\\d+\\.\\d+` |
| `toml`         | Dot-separated path to the version number in the TOML file. For example: `dependencies.server.version`                                                                                                           |

Replace formats for different dependent types:

| Dependent type | Replace format                                                                                                                         |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`        | A format string that replaces the matching text in the dependent file. Defaults to the new version string. For example: `v{{version}}` |
| `toml`         | N/A                                                                                                                                    |

The `replace` string can contain the `{{version}}` placeholder, which will be replaced with the new version number when the project is released.

### Example YAML configuration

```yaml
projects:
  server:
    type: rust
    path: server
    dependents:
      - type: regex
        path: client/package.json
        selector: "\"server\": \".?\\d+\\.\\d+\\.\\d+\""
        replace: '"server": "{{version}}"'
      - type: toml
        path: dependency.toml
        selector: dependencies.server.version
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

[[projects.server.dependents]]
type = "regex"
path = "client/package.json"
selector = "\"server\": \".?\\d+\\.\\d+\\.\\d+\""
replace = '"server": "{{version}}"'

[[projects.server.dependents]]
type = "toml"
path = "dependency.toml"
selector = "dependencies.server.version"

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

### Release

You can create a new version by running `monoverse release` command

```bash
monoverse release <project>
```

where `<project>` is the key of the project as defined in the configuration file.

Monoverse will then check if the project has been modified since the last release. If there are changes, it will craft a new version number and update the project's manifest file depending on its type.

If the project has dependents, Monoverse will also update the dependent files with the new version number.

The manifest file must not have any uncommitted changes, or the release will fail.

### Next

You can also get the next version number without actually updating the project's manifest file by running the `monoverse next` command

```bash
monoverse next <project>
```

where `<project>` is the key of the project as defined in the configuration file. The program will then print the next version number to the standard output if a new release is required.
