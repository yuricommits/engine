// Copyright (C) 2026 Yuri
// Licensed under GPL-3.0

use calculus::differential::derivatives::{derivative, nth_derivative, second_derivative};

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
    #[inline(always)]
    pub fn position_at(&self, t: f64) -> f64 {
        (self.x)(t)
    }

    /// Calculates the displacement vector component: $\Delta x = x(t_2) - x(t_1)$.
    #[inline(always)]
    pub fn displacement(&self, t1: f64, t2: f64) -> f64 {
        self.position_at(t2) - self.position_at(t1)
    }

    /// Calculates the average velocity component: $\bar{v} = \frac{\Delta x}{\Delta t}$.
    /// Returns `NaN` if &t_1 = t_2& due to division by zero.
    #[inline(always)]
    pub fn average_velocity(&self, t1: f64, t2: f64) -> f64 {
        self.displacement(t1, t2) / (t2 - t1)
    }

    /// Calculates the instantaneous velocity $v(t) = \frac{dx}{dt}$.
    #[inline(always)]
    pub fn velocity_at(&self, t: f64) -> f64 {
        derivative(&self.x, t)
    }

    /// Calculates the instantaneous acceleration $a(t) = \frac{dv}{dt}$.
    #[inline(always)]
    pub fn acceleration_at(&self, t: f64) -> f64 {
        second_derivative(&self.x, t)
    }

    /// Calculates the arbitrary kinematic order at time $t$.
    ///
    /// * N=3: Jerk ($j$)
    /// * N=4: Snap ($s$)
    ///
    /// This utilizes the zero-overhead const-generic stencils.
    #[inline(always)]
    pub fn kinematic_order_at<const N: usize>(&self, t: f64) -> f64 {
        nth_derivative::<_, N>(&self.x, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nan_on_zero_interval() {
        let p = Particle1d::new(|t| 5.0 * t);
        assert!(p.average_velocity(2.0, 2.0).is_nan())
    }

    #[test]
    fn test_linear_motion() {
        let p = Particle1d::new(|t| 10.0 * t + 5.0);
        assert_eq!(p.position_at(0.0), 5.0);
        assert_eq!(p.displacement(0.0, 2.0), 20.0);
        assert_eq!(p.average_velocity(0.0, 2.0), 10.0);
        assert!((p.velocity_at(1.0) - 10.0).abs() < 1e-8)
    }

    #[test]
    fn test_higher_order_kinematics() {
        // x(t) = t^4 + t^3
        // v(t) = 4t^3 + 3t^2 -> at t=2: 32 + 12 = 44
        // a(t) = 12t^2 + 6t  -> at t=2: 48 + 12 = 60
        // j(t) = 24t + 6     -> at t=2: 48 + 6 = 54
        // s(t) = 24          -> at t=2: 24

        let p = Particle1d::new(|t| t.powi(4) + t.powi(3));
        let t = 2.0;

        assert!((p.velocity_at(t) - 44.0).abs() < 1e-7);
        assert!((p.acceleration_at(t) - 60.0).abs() < 1e-6);

        // Testing Jerk (N=3) and Snap (N=4)
        assert!((p.kinematic_order_at::<3>(t) - 54.0).abs() < 1e-5);
        assert!((p.kinematic_order_at::<4>(t) - 24.0).abs() < 1e-4);
    }
}
