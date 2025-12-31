# Configuration

Monoverse reads configuration from `monoverse.{yaml,json,toml}` at the repository root.

## Project settings

Each project is defined in the `projects` map. The project key is the project name used in CLI commands.

| Key             | Description                                   | Notes |
| --------------- | --------------------------------------------- | ----- |
| `type`          | The type of the project.                      | `rust`, `node`, `helm`, `toml`, `versionfile`, `yaml` |
| `path`          | The path to the project.                      | Defaults to repository root if omitted. |
| `manifest_path` | The path to the manifest file of the project. | Overrides the default manifest path. |
| `tag_prefix`    | Prefix for tag creation.                      | Defaults to `<project-name>-`. |
| `selector`      | Selector for the version field.               | Required for `toml` and `yaml` project types. |
| `dependents`    | Dependent files to update on release.         | See [Dependents](dependents.md). |

### Project types

- `rust`: A Rust project with a `Cargo.toml` manifest file.
- `node`: A Node.js project with a `package.json` manifest file.
- `helm`: A Helm chart with a `Chart.yaml` manifest file. Uses `appVersion` for release detection and updates `appVersion` plus the chart `version`.
- `toml`: Any generic project with a TOML manifest file. Requires `manifest_path` and `selector`.
- `versionfile`: Any generic project with a version file that only contains the version number. Requires `manifest_path`.
- `yaml`: Any generic project with a YAML manifest file. Requires `manifest_path` and `selector`.

## Examples

YAML:

```yaml
projects:
  server:
    type: rust
    path: server
  client:
    type: node
    path: client
```

TOML:

```toml
[projects.server]
type = "rust"
path = "server"

[projects.client]
type = "node"
path = "client"
```
