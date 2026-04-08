// Copyright (C) 2026 Yuri
// Licensed under GPL-3.0

use calculus::differential::derivatives::derivative;

/// Represents a point-mass particle moving in a one-dimensional Cartesian coordinate system.
pub struct Particle1d<F>
where
    F: Fn(f64) -> f64,
{
    /// The position function $x(t)$
    pub x: F,
}

impl<F> Particle1d<F>
where
    F: Fn(f64) -> f64,
{
    /// Creates a new `Particle1d` with the given position function.
    pub fn new(x: F) -> Self {
        Self { x }
    }

    /// Returns the position coordinate at a specific instant in time $t$.
    pub fn position_at(&self, t: f64) -> f64 {
        (self.x)(t)
    }

    /// Calculates the displacement vector component: $\Delta x = x(t_2) - x(t_1)$.
    pub fn displacement(&self, t1: f64, t2: f64) -> f64 {
        self.position_at(t2) - self.position_at(t1)
    }

    /// Calculates the average velocity component: $\bar{v} = \frac{\Delta x}{\Delta t}$.
    /// Returns `NaN` if &t_1 = t_2&
    pub fn average_velocity(&self, t1: f64, t2: f64) -> f64 {
        self.displacement(t1, t2) / (t2 - t1)
    }

    /// Calculates the instantaneous velocity $v(t) = \frac{dx}{dt}$.
    pub fn velocity_at(&self, t: f64) -> f64 {
        derivative(&self.x, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nan_on_zero_interval() {
        let p = Particle1d::new(|t| 5.0 * t);
        // Average velocity at a single point should be NaN (0/0)
        assert!(p.average_velocity(2.0, 2.0).is_nan())
    }

    #[test]
    fn test_linear_motion() {
        let p = Particle1d::new(|t| 10.0 * t + 5.0);
        assert_eq!(p.position_at(0.0), 5.0);
        assert_eq!(p.displacement(0.0, 2.0), 20.0);
        assert_eq!(p.average_velocity(0.0, 2.0), 10.0);
        // Instantaneous velocity of 10t + 5 is 10
        assert!((p.velocity_at(1.0) - 10.0).abs() < 1e-8)
    }
}
