# Quick start

Install monoverse first if you haven't already. See [Installation](installation.md).

Create a `monoverse.yaml` file at the repository root, define your projects, then run a release.

Example:

```yaml
projects:
  server:
    type: rust
    path: server
  client:
    type: node
    path: client
```

Release a project:

```bash
monoverse release server
```

Add the `--commit` and `--tag` flags if you want Monoverse to handle committing and tagging the changes.

Print the next version without editing files:

```bash
monoverse next server
```
