# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Deprecated

### Breaking

### Added
- Update the template to the latest-released `esp-idf-svc` (0.51)
- Update the template to the latest-released `esp-idf-svc` (0.50)
- Update the template to ESP-IDF V5.3 (latest stable)

### Fixed
- Fixed the generated GH CI by making sure `ldproxy`, `cargo fmt`, `cargo clippy` and rust src for `-Zbuild-std` are operational with the nightly toolchain (#253)
- Update the README to specify 1.85.0 as the version to install with espup so core and std compile properly
