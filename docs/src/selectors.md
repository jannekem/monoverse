# Selectors

Selectors point to the version field in `yaml` and `toml` projects and in `yaml`/`helm` dependents.

## YAML selectors

- Dot-separated keys: `package.version`
- Sequence index: `dependencies[0].version`
- Sequence filter by key/value: `dependencies[name=common].version`

Examples:

```yaml
projects:
  common:
    type: yaml
    manifest_path: charts/common/Chart.yaml
    selector: version
```

```yaml
dependents:
  - type: helm
    path: charts/laravel
    selector: dependencies[name=common].version
```

Notes:

- The selector must resolve to a scalar value.
- Sequence filters return the first matching item.
- Keys containing `.` or `[` are not supported by the selector syntax.

## TOML selectors

Use dot-separated keys:

- `package.version`
- `dependencies.serde.version`
