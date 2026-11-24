# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Deprecated

### Breaking

### Added
- Update the template to the latest-released `esp-idf-svc` (0.54)
- Update the list of supported ESP-IDF versions (add 6.0.x and 6.1, bump 5.x patch levels)
- Update the template to the latest-released `esp-idf-svc` (0.53)
- Update the template to the latest-released `esp-idf-svc` (0.52)
- Enable Embassy and `critical-section` support by default
- Update the template to the latest-released `esp-idf-svc` (0.51)
- Update the template to the latest-released `esp-idf-svc` (0.50)
- Update the template to ESP-IDF V5.3 (latest stable)

### Fixed
- Pass `--cfg espidf_picolibc` with picolibc (the default since ESP-IDF v6.0), which `esp-idf-sys` 0.39 requires
- Without the advanced options, use ESP-IDF v5.5.5 (as with them), rather than v5.5.3
- cmake template: add `lwip` to the ESP-IDF components of the Rust component, as `esp-idf-svc` 0.54 does not build without it
- Pin the nightly toolchain to `nightly-2026-10-19`, as newer nightlies cannot build `std` for ESP-IDF (rust-lang/rust#164555)
- Fixed the generated GH CI by making sure `ldproxy`, `cargo fmt`, `cargo clippy` and rust src for `-Zbuild-std` are operational with the nightly toolchain (#253)
