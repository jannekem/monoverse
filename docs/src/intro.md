# Introduction

Monoverse is a tool for managing application version numbering using the CalVer versioning scheme. It is designed to be used with monorepos, but it can also be used with single projects.

The monoverse implementation of CalVer follows the `YY.MM.MICRO` format, where `YY` is the current year, `MM` is the current month, and `MICRO` is a monotonically increasing number that is reset to `0` at the beginning of each month.

Monoverse updates manifest files based on project type and uses git history to decide whether a new release is needed.
