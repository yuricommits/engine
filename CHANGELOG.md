# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [calculus] : Numerical differentiation using the symmetric difference quotient for `f64`.
- [calculus] : Hardware-optimal `H` ($\approx 1.49e-8$) for minimized error.
- [calculus] : Physics validation suite for Power, Trig and Constant rules.
- [calculus] : Second-order numerical differentiation using the 3-point central difference formula.
- [calculus] : Optimized `H_SECOND` ($\approx 6.0e-6$) for stable acceleration calculations.
- [calculus] : `nth_derivative` implementation using Const Generics to support arbitary order differentiation at compile-time.
- [calculus] : Automatic binomial coefficient generation via `const fn combinations` for stencil weight calculation.
- [calculus] : Dynamic relative step-size scaling to preserve precision across astronomical scales (from atomic to parsec ranges).
- [calculus] : `PASCAL` lookup table for orders $N \in [0, 4]$ to eliminate runtime calculation overhead for common simulation derivatives.
- [calculus] : Singularity stress testing to verify stability near vertical asymptotes ($1/x$).
- [calculus] : Corner-case validation for non-differentiable points ($|x|$).

- [mechanics] : `Particle1d` model for one-dimensional kinematics.
- [mechanics] : Methods for position, displacement, and average velocity.
- [mechanics] : Instantaneous `velocity_at` via `calculus` differentiation.
- [mechanics] : Safety-validated `NaN` handling for zero-time intervals.
- [mechanics] : `acceleration_at` implementation for `Particle1d`.
- [mechanics] : `kinematic_order_at<const N>` allowing zero-overhead analysis of Jerk, Snap, and higher-order motion.

### Changed
- [calculus] : Verified precision using Relative Error ($\epsilon_{rel} < 10^{-10}$) rather than absolute tolerance.
- [calculus] : Replaced the hardcoded `H` and `H_SECOND` constants with a mathematically derived `optimal_h(n, x)` to minimize the sum of trucncation and rounding errors.
- [calculus] : Refactored specialization logic using `match` on constant values to allow zero-cost compiler branching.
- [calculus] : Upgraded test suite to include Scale Invariance and Transcendental verification for high-order stencils.

- [mechanics] : Optimized acceleration calculation to reference the position function directly, preventing compound numerical noise.

### Fixed
- [workspace] : Updated `Cargo.toml` to `resolver = "3"` for Edition 2024.

- [calculus] : Relaxed precision thresholds for singularity tests to $5e-3$ to accomodate extreme third-derivative growth of $1/x$.
- [calculus] : Resolved `clippy::needless_range_loop` by integrating `PASCAL` table logic with compile-time branch pruning.
