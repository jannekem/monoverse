# Quick start

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

Print the next version without editing files:

```bash
monoverse next server
```
