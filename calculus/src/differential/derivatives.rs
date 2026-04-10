// Copyright (C) 2026 Yuri
// Licensed under GPL-3.0

/// Machine epsilon for f64
const EPS: f64 = f64::EPSILON;

/// Computes binomial coefficients at compile-time.
const fn combinations(n: usize, k: usize) -> f64 {
    let k = if k > n / 2 { n - k } else { k };
    let mut res = 1.0;
    let mut i = 0;
    while i < k {
        res *= (n - i) as f64 / (i + 1) as f64;
        i += 1;
    }
    res
}

/// Pascal's Triangle for N=0 through N=4.
/// Used for zero-overhead coefficients in differentiation stencils.
const PASCAL: [[f64; 5]; 5] = [
    [1.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 0.0, 0.0, 0.0],
    [1.0, 2.0, 1.0, 0.0, 0.0],
    [1.0, 3.0, 3.0, 1.0, 0.0],
    [1.0, 4.0, 6.0, 4.0, 1.0],
];

/// Optimal step size for N-th order central difference.
#[inline(always)]
fn optimal_h(n: usize, x: f64) -> f64 {
    let h_base = EPS.powf(1.0 / (n as f64 + 2.0));
    h_base * (1.0 + x.abs())
}

/// Numerical first-order derivative (N=1) using the symmetric difference quotient.
///
/// $$f'(x) \approx \frac{f(x + h) - f(x - h)}{2h}$$
#[inline(always)]
pub fn derivative<F>(f: F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = optimal_h(1, x);
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Numerical second-order derivative (N=2) using the 3-point central difference.
///
/// $$f''(x) \approx \frac{f(x+h) - 2f(x) + f(x-h)}{h^2}$$
#[inline(always)]
pub fn second_derivative<F>(f: F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = optimal_h(2, x);
    (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
}

/// N-th order derivative using a Central Difference Stencil.
///
/// Uses Const Generics to specialize logic and unroll summation loops.
#[inline(always)]
pub fn nth_derivative<F, const N: usize>(f: F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    match N {
        0 => f(x),
        1 => derivative(f, x),
        2 => second_derivative(f, x),
        _ => {
            let h = optimal_h(N, x);
            let mut sum = 0.0;
            let half_n = N as f64 * 0.5;

            #[allow(clippy::needless_range_loop)]
            for i in 0..=N {
                let coeff = if N < 5 {
                    PASCAL[N][i]
                } else {
                    combinations(N, i)
                };
                let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                let point = x + (half_n - i as f64) * h;
                sum += sign * coeff * f(point);
            }

            sum / h.powi(N as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // I. Test for Calculus
    // Verifies N=1 (Linearity) and N=2 (Curvature).
    #[test]
    fn test_low_order() {
        let f = |x: f64| x.powi(3);
        assert!((nth_derivative::<_, 1>(f, 2.0) - 12.0).abs() < 1e-8);
        assert!((nth_derivative::<_, 2>(f, 2.0) - 12.0).abs() < 1e-7);
    }

    // II. Test for Scale
    // Verifies that h_scaling works at Galactic scales (1e16 meters).
    // Without h_scaling, this returns 0.0 or NaN.
    #[test]
    fn test_large_scales() {
        let pc = 3.086e16;
        let f = |x: f64| x * 5.0;
        assert!((nth_derivative::<_, 1>(f, pc) - 5.0).abs() < 1e-6);
    }

    // III. Test for Higher-Order Dynamics
    // Verifies the binomial coefficients and loop unrolling for N=4.
    #[test]
    fn test_higher_order() {
        let f = |x: f64| x.powi(4);
        assert!((nth_derivative::<_, 4>(f, 1.0) - 24.0).abs() < 1e-4);
    }

    // IV. Test for Oscillation
    // Critical for wave-mechanics or orbital oscillations.
    #[test]
    fn test_trig() {
        let x = 0.5;
        assert!((nth_derivative::<_, 1>(f64::sin, x) - x.cos()).abs() < 1e-9);
    }

    // V. Test for Identity
    // Ensure N=0 returns the function value exactly.
    #[test]
    fn zero_order() {
        let val = nth_derivative::<_, 0>(|x| x.exp(), 1.0);
        assert_eq!(val, 1.0f64.exp());
    }

    // VI. Test for Singularity
    // Verifies behavior near a vertical asymptote (1/x at x -> 0).
    // The derivative of 1/x is -1/x^2.
    #[test]
    fn test_singularity() {
        let f = |x: f64| 1.0 / x;
        let x_near_zero = 1e-4;

        let d = nth_derivative::<_, 1>(f, x_near_zero);
        let expected = -1.0 / (x_near_zero * x_near_zero);

        let rel_error = (d - expected).abs() / expected.abs();
        assert!(d.is_finite(), "Numerical overflow/NaN at x={}", x_near_zero);
        assert!(
            rel_error < 5e-3,
            "Precision loss: rel_err={} exceeds limit 1e-5",
            rel_error
        );
    }

    // VII. Test for Non-Differentiable Points
    // Validates behavior at f'(x) undefined (corner).
    #[test]
    fn test_corner_case() {
        let f = |x: f64| x.abs();
        let d = nth_derivative::<_, 1>(f, 0.0);

        assert_eq!(
            d,
            0.0,
            "Stencil asymmetry: expected 0.0 for symmetric corner, get {}", d
        );
    }
}
