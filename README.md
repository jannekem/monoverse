# monoverse

## Introduction

Monoverse is a tool for managing application version numbering using the [CalVer](https://calver.org/) versioning scheme. It is designed to be used with monorepos, but it can also be used with single projects as witnessed by the `monoverse` project itself.

The reality for many projects is that they don't actually need to follow [semantic versioning](https://semver.org/). For example, if you are building a web application with lots of microservices, you're generally not going to be publishing them as libraries. Instead, they'll be deployed as individual APIs that each have their own lifecycle.

CalVer is a simple versioning scheme that is based on the calendar. The `monoverse` implementation of CalVer follows the `YY.MM.MICRO` format, where `YY` is the current year, `MM` is the current month, and `MICRO` is a monotonically increasing number that is reset to `0` at the beginning of each month.

[Ubuntu](https://wiki.ubuntu.com/Releases) is a famous example of a project that uses (a variation of) CalVer. Just remember that it is not a replacement for semantic versioning. If you're building a library, you should probably stick to semantic versioning as going back from CalVer is not easy.

## Configuration

Monoverse can be configured by defining a configuration file at the root of the project. The supported file formats are `json`, `yaml` and `toml`. The configuration file must be named `monoverse.{json,yaml,toml}`.

The configuration file defines the projects that are part of the monorepo. Each project is defined by a type and a path.

### Example

```yaml
projects:
  server:
    type: node
    path: server
  client:
    type: node
    path: client
```
