# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [calculus] : Numerical differentiation using the symmetric difference quotient for `f64`.
- [calculus] : Hardware-optimal `H` ($\approx 1.49e-8$) for minimized error.
- [calculus] : Physics validation suite for Power, Trig and Constant rules.
- [mechanics] : `Particle1d` model for one-dimensional kinematics.
- [mechanics] : Methods for position, displacement, and average velocity.
- [mechanics] : Instantaneous `velocity_at` via `calculus` integration.
- [mechanics] : Safety-validated `NaN` handling for zero-time intervals.

### Changed
- [calculus] : Verified precision using Relative Error ($\epsilon_{rel} < 10^{-10}$) rather than absolute tolerance.

### Fixed
- [workspace] : Updated `Cargo.toml` to `resolver = "3"` for Edition 2024.
