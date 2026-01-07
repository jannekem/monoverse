# Helm workflows

Monoverse supports Helm charts both as projects and as dependents.

## Helm projects

`type: helm` expects a chart with `Chart.yaml` and uses `version` for release detection. On release it updates:

- `version` to the next CalVer
- `appVersion` if present

The versioning is similar for both application and library charts.

## Helm dependents

Use `type: helm` when a release should update a chart dependency version.

- `path` is the chart directory, not `Chart.yaml`.
- The selector targets the dependency entry in `Chart.yaml`.

Example:

```yaml
projects:
  common:
    type: yaml
    manifest_path: charts/common/Chart.yaml
    selector: version
    dependents:
      - type: helm
        path: charts/laravel
        selector: dependencies[name=common].version
```

### Updating Chart.lock

`monoverse` can run `helm dependency update` when releasing to update `Chart.lock` and `charts/*.tgz`:

```bash
monoverse release common --helm-dependency-update
```

This requires Helm on `PATH` and may fetch dependencies.
