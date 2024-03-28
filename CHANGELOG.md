# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
