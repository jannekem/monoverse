# Dependents

Dependents are files that are updated when a project is released.

## Dependent settings

| Key        | Description                                                | Notes |
| ---------- | ---------------------------------------------------------- | ----- |
| `type`     | The type of the dependent.                                 | `regex`, `toml`, `yaml`, `helm` |
| `path`     | The path to the dependent file.                            | For `helm`, use the chart directory. |
| `selector` | Selector for the version number in the dependent file.     | Required for `toml`, `yaml`, `helm`. |
| `replace`  | String to replace the selector match with.                 | Only for `regex`. |

## Dependent types

### regex

Uses a regex to replace a version string. The `{{version}}` special sequence will be replaced with the next version.

```yaml
dependents:
  - type: regex
    path: README.md
    selector: "nginx:.?\\d+\\.\\d+\\.\\d+"
    replace: "nginx:{{version}}"
```

### toml

```yaml
dependents:
  - type: toml
    path: dependency.toml
    selector: dependencies.server.version
```

### yaml

```yaml
dependents:
  - type: yaml
    path: deploy/config.yaml
    selector: package.version
```

### helm

```yaml
dependents:
  - type: helm
    path: charts/laravel
    selector: dependencies[name=common].version
```

If you need `Chart.lock` updates, use the `--helm-dependency-update` CLI option or run `helm dependency update` manually.
