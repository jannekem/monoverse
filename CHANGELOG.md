# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.10](https://github.com/jannekem/monoverse/compare/v0.1.9...v0.1.10) - 2026-11-27

### Fixed

- *(deps)* upgrade git2 to 0.21.0 ([#37](https://github.com/jannekem/monoverse/pull/37))
- *(helm)* respect ignored dependency archives when committing ([#30](https://github.com/jannekem/monoverse/pull/30))

### Other

- *(docker)* update runtime image and trim packages ([#39](https://github.com/jannekem/monoverse/pull/39))
- *(deps)* update Rust dependencies ([#38](https://github.com/jannekem/monoverse/pull/38))
- remove duplicate Rust cache step ([#36](https://github.com/jannekem/monoverse/pull/36))
- migrate remaining actions to Node.js 24 ([#35](https://github.com/jannekem/monoverse/pull/35))
- *(deps)* track Cargo.lock and enforce locked builds ([#34](https://github.com/jannekem/monoverse/pull/34))
- *(deps)* remove unused thiserror dependency ([#33](https://github.com/jannekem/monoverse/pull/33))
- upgrade checkout action to Node.js 24 ([#32](https://github.com/jannekem/monoverse/pull/32))

## [0.1.9](https://github.com/jannekem/monoverse/compare/v0.1.8...v0.1.9) - 2026-01-09

### Fixed

- improve toml selector errors
- prevent toml selector panics

### Other

- update Rust in Dockerfile
- Merge pull request #27 from jannekem/fix/toml-panic

## [0.1.8](https://github.com/jannekem/monoverse/compare/v0.1.7...v0.1.8) - 2026-01-08

### Added

- add list command
- update helm to use version instead of appVersion
- *(helm)* add helm dependents and sequence selectors

### Fixed

- use clap command attributes
- bump toml_edit to 0.24.0
- normalize manifest paths for git2

### Other

- drop serde_yaml dependency
- bump serde_json to 1.0.149
- bump serde to 1.0.228
- bump regex to 1.12.2
- bump log to 0.4.29
- bump clap to 4.5.54
- bump chrono to 0.4.42
- bump anyhow to 1.0.100
- bump thiserror to 2.0.17
- bump stderrlog to 0.6.0
- bump libyaml-safer to 0.3.0
- bump git2 to 0.20.3
- bump config to 0.15.19
- bump clap-verbosity-flag to 3.0.4
- document list feature
- streamline documentation
- fix release pipeline
- add mdbook documentation

## [0.1.7](https://github.com/jannekem/monoverse/compare/v0.1.6...v0.1.7) - 2024-09-10

### Added

- add versionfile project type

### Other

- add missing dependent type

## [0.1.6](https://github.com/jannekem/monoverse/compare/v0.1.5...v0.1.6) - 2024-05-21

### Added
- add commit and tag options

### Other
- reorganize installation instructions
- unify dependent handling
- fix clippy warnings
- organize subcommand handlers in separate functions

## [0.1.5](https://github.com/jannekem/monoverse/compare/v0.1.4...v0.1.5) - 2024-03-28

### Added
- include git in the container image

## [0.1.4](https://github.com/jannekem/monoverse/compare/v0.1.3...v0.1.4) - 2024-03-17

### Added
- make logging verbosity configurable
- add --force flag to release command

### Other
- mention force flag in README
- add container instructions

## [0.1.3](https://github.com/jannekem/monoverse/compare/v0.1.2...v0.1.3) - 2024-03-08

### Added
- add container support
- add YAML dependent

### Fixed
- apply cargo clippy fixes

### Other
- use PAT to allow triggering Docker build
- check clippy

## [0.1.2](https://github.com/jannekem/monoverse/compare/v0.1.1...v0.1.2) - 2024-02-28

### Added
- print new version
- add YAML project type
- implement yaml edit using libyaml_safer
- add yaml query using libyaml_safer

### Other
- run tests on push to main
- run unit tests
- use edit::yaml for helm project

## [0.1.1](https://github.com/jannekem/monoverse/compare/v0.1.0...v0.1.1) - 2024-02-02

### Added
- add TOML project
- add regex dependent
- add TOML dependents

### Other
- separate io operations
- add README badges
- update installation instructions
