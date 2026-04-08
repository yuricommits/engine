// Copyright (C) 2026 Yuri
// Licensed under GPL-3.0

/// The optimal step size for a symmetric difference quotient in f64-bit floats.
/// Calculated as sqrt(f64::EPSILON) = 1.49011611938476562500e-8
const H: f64 = 1.49011611938476562500e-8;

/// Optimal step size for second-order numerical differentiation in f64.
const H_SECOND: f64 = 6.0e-6;

/// Numerical first-order derivative using the symmetric difference quotient.
///
/// $$f'(x) \approx \frac{f(x + h) - f(x - h)}{2h}$$
///
/// # Numerical Properties
/// - Accuracy: $\mathcal{O}{h^2}$
/// - Step Size: $H = \sqrt{\epsilon} \approx 1.49 \times 10^{-8}$
pub fn derivative<F>(f: F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + H) - f(x - H)) / (2.0 * H)
}

/// Calculates the second derivative f''(x) using the 3-point central difference.
///
/// $$f''(x) \approx \frac{f(x+h) - 2f(x) + f(x-h)}{h^2}$$
pub fn second_derivative<F>(f: F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + H_SECOND) - 2.0 * f(x) + f(x - H_SECOND)) / (H_SECOND * H_SECOND)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_rule() {
        // f(x) = x^2 => f'(x) = 2x
        // At x = 5.0, f'(5.0) = 10.0
        let f = |x: f64| x.powi(2);
        let expected: f64 = 10.0;
        let res = derivative(f, 5.0);

        let rel_error = (res - expected).abs() / expected.abs();
        assert!(rel_error < 1e-10);
    }

    #[test]
    fn test_trig_derivative() {
        // f(x) = sin(x) => f'(x) = cos(x)
        // At x = 0.0, cos(0) = 1.0
        let f = |x: f64| x.sin();
        let res = derivative(f, 0.0);
        assert!((res - 1.0).abs() < 1e-8);
    }

    #[test]
    fn test_constant_rule() {
        // f(x) = 42 => f'(x) = 0
        let f = |_x: f64| 42.0;
        let res = derivative(f, 1.5);
        assert!(res.abs() < 1e-8);
    }
}
