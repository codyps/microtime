# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added checked constructors and arithmetic methods for values that may exceed the
  representable microsecond range.
- Added `Duration::as_micros` and `TimeRangeError`.

### Changed

- Replaced the panic-prone `From` conversions between `RealTime` and `SystemTime`
  with fallible `TryFrom` conversions.
- Arithmetic operators now panic consistently on overflow or underflow instead of
  silently wrapping in optimized builds.

## [0.3.0](https://github.com/codyps/microtime/releases/tag/microtime-0.3.0) - 2021-12-05

### Added

- Added `no_std` support and a microsecond-resolution `Duration` type.
- Added optional `std::time::SystemTime` interoperability.

