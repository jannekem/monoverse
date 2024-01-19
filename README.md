# monoverse

## Configuration

Monoverse can be configured by defining a configuration file at the root of the project. The supported file formats are `json`, `yaml` and `toml`. The configuration file name must be named `monoverse.{json,yaml,toml}`.

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
