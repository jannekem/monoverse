# Introduction

[![GitHub stars](https://img.shields.io/github/stars/jannekem/monoverse?style=social)](https://github.com/jannekem/monoverse)
[![Crates.io Version](https://img.shields.io/crates/v/monoverse.svg)](https://crates.io/crates/monoverse)
![Crates.io Total Downloads](https://img.shields.io/crates/d/monoverse)
[![GitHub License](https://img.shields.io/crates/l/monoverse.svg)](https://github.com/jannekem/monoverse/blob/main/LICENSE)

Monoverse is a tool for managing application version numbering using the CalVer versioning scheme. It is designed to be used with monorepos, but it can also be used with single projects.

The monoverse implementation of CalVer follows the `YY.MM.MICRO` format, where `YY` is the current year, `MM` is the current month, and `MICRO` is a monotonically increasing number that is reset to `0` at the beginning of each month.

Monoverse updates manifest files based on project type and uses git history to decide whether a new release is needed.
