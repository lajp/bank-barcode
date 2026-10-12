# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- The barcode version is now detected from the reference number when it is not
  set explicitly with `BarcodeBuilder::version`: references starting with `RF`
  produce a V5 barcode and other references a V4 barcode. Without a reference
  the version still defaults to V5. Previously a non-`RF` reference without an
  explicit version failed with `BuilderError::MalformedReference`.

## [0.2.0] – 2026-04-11

### Fixed

- Removed a leftover `dbg!()` macro in the V5 `Display` implementation that
  printed every formatted barcode to stderr in release builds.

### Changed

- **Breaking:** bumped `iban` dependency from `0.1.7` to `0.2`. Because
  `bank-barcode`'s public API re-exports `iban::Iban` (via
  `BarcodeBuilder::account_number_iban` and `BuilderError::InvalidAccount`),
  this is a breaking change for downstream users who depend on `iban` directly.
- Bumped other dependencies to their latest compatible versions: `either` to
  `1.15`, `thiserror` to `2.0.18`, `time` to `0.3.47`.
- Declared `rust-version = "1.88"` as the minimum supported Rust version.
- Opted into `clippy::pedantic` at warn level and addressed the resulting
  warnings; added `#[must_use]` to all builder methods.

### Added

- `README.md` with installation and usage instructions.
- `examples/basic.rs` runnable example.
- Crates.io discoverability metadata in `Cargo.toml` (`repository`,
  `documentation`, `readme`, `keywords`, `categories`).
- Documentation comments on the crate root and on every `BuilderError` variant.

## [0.1.0] – 2025-03-30

### Added

- Initial release: V4 and V5 Finnish virtual bank barcode generation through a
  builder API.
