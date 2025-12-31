# Troubleshooting

## "Key not found"

The selector did not resolve to a scalar value. Verify the selector path and ensure the key exists.

## "Value is not a mapping" or "Value is not a sequence"

The selector expects a mapping or sequence but found a different YAML node type. Check the structure of the file.

## Helm dependency update fails

- Ensure Helm is installed and available on `PATH`.
- The chart directory must be valid and contain `Chart.yaml`.
- If dependencies are remote, network access is required.

## Version file modified

Monoverse refuses to release if the manifest file has uncommitted changes. Commit or stash the file, then retry.
