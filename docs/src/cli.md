# CLI

```text
Usage: monoverse [OPTIONS] <COMMAND>

Commands:
  release  Release a project
  next     Print the next version for a project
  help     Print this message or the help of the given subcommand(s)

Options:
      --repo-path <REPO_PATH>  Repository path [default: .]
  -v, --verbose...             Increase logging verbosity
  -q, --quiet...               Decrease logging verbosity
  -h, --help                   Print help
  -V, --version                Print version
```

## release

Create a new version for a project.

```bash
monoverse release <project>
```

Flags:

- `-f`, `--force`: Force a release even if the project has no changes.
- `--commit`: Commit the changes to the repository.
- `--tag`: Create a tag, requires `--commit`.
- `--helm-dependency-update`: Run `helm dependency update` for `helm` dependents.

## next

Print the next version without modifying files:

```bash
monoverse next <project>
```

## repo path

Use `--repo-path` to run from outside the repository root:

```bash
monoverse --repo-path /path/to/repo release server
```
