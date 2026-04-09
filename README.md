# Physics & Computational Astrophysics Library (Rust)

[![License: GPL-3.0](https://img.shields.io/badge/Licence-GPL--3.0-white.svg)](https://opensource.org/license/gpl-3.0)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust-2024-black.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

A high-performance strictly typed computational engine for classical mechanics and astrophysical simulations. This library implements formal mathematical structures to model physical systems with a focus on numerical stability and transparency.

## Architecture

The workspace is organized into discrete domains to ensure mathematical rigor and physical consistency:

### `Calculus` (Numerical Analysis)
The analytical engine of the library. It provides arbitrary-order numerical differentiation using compile-time specialized stencils and scale-invariant step sizes. Optimized for IEEE 754 double-precision math, the module is engineered to preserve precision across astronomical scales by minimizing the interplay between truncation and round-off errors.    

### `Mechanics` (Physical Systems)
The application layer mapping mathematical abstractions to physical observables. This module handles the kinematics and dynamics of point-mass rigid bodies, providing a functional interface for position-based motion, force-vector summation, and orbital trajectories.

## Documentation
```zsh
cargo doc --workspace --open
```

## Licensed
Copyright (C) 2026 Yuri

This project is licenced under the `GNU General Public License v3.0`.
See the [LICENSE](https://github.com/yuricommits/engine/blob/main/LICENSE) file for details.
