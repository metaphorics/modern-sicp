// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.11: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_11 {
    use ch02::sec_2_1::Interval;

    /// The sign of an interval's endpoints, for Ben's nine-case dispatch.
    enum Sign {
        /// Both bounds are nonnegative.
        NonNeg,
        /// Both bounds are nonpositive.
        NonPos,
        /// The bounds straddle zero.
        Mixed,
    }

    fn sign(lower: f64, upper: f64) -> Sign {
        if lower >= 0.0 {
            Sign::NonNeg
        } else if upper <= 0.0 {
            Sign::NonPos
        } else {
            Sign::Mixed
        }
    }

    /// Exercise 2.11: `mul-interval` rewritten with Ben Bitdiddle's nine
    /// sign-tested cases, only one of which (both intervals spanning
    /// zero) needs more than two multiplications.
    pub fn mul_interval_fast(x: &Interval, y: &Interval) -> Interval {
        let (xl, xu) = (x.lower_bound(), x.upper_bound());
        let (yl, yu) = (y.lower_bound(), y.upper_bound());

        let (lower, upper) = match (sign(xl, xu), sign(yl, yu)) {
            (Sign::NonNeg, Sign::NonNeg) => (xl * yl, xu * yu),
            (Sign::NonNeg, Sign::NonPos) => (xu * yl, xl * yu),
            (Sign::NonNeg, Sign::Mixed) => (xu * yl, xu * yu),
            (Sign::NonPos, Sign::NonNeg) => (xl * yu, xu * yl),
            (Sign::NonPos, Sign::NonPos) => (xu * yu, xl * yl),
            (Sign::NonPos, Sign::Mixed) => (xl * yu, xl * yl),
            (Sign::Mixed, Sign::NonNeg) => (xl * yu, xu * yu),
            (Sign::Mixed, Sign::NonPos) => (xu * yl, xl * yl),
            (Sign::Mixed, Sign::Mixed) => {
                let products = [xl * yl, xl * yu, xu * yl, xu * yu];
                (
                    products.into_iter().fold(f64::INFINITY, f64::min),
                    products.into_iter().fold(f64::NEG_INFINITY, f64::max),
                )
            }
        };

        Interval::new(lower, upper)
            .expect("each of the nine cases above pairs a provably smaller product with a provably larger one")
    }

    /// Exercise 2.11: `mul-interval` by nine sign-tested cases
    ///
    /// Returns the bounds of `[2, 6] * [-3, 5]` (a case that spans
    /// zero), computed by the sign-tested procedure.
    pub fn ex_2_11() -> (f64, f64) {
        let x = Interval::new(2.0, 6.0).expect("2.0 <= 6.0");
        let y = Interval::new(-3.0, 5.0).expect("-3.0 <= 5.0");
        let product = mul_interval_fast(&x, &y);
        (product.lower_bound(), product.upper_bound())
    }
}

#[cfg(test)]
mod tests {
    use ch02::sec_2_1::{Interval, mul_interval};
    use proptest::prelude::*;

    use super::ex_2_11::{ex_2_11 as ex_2_11_answer, mul_interval_fast};

    #[test]
    fn ex_2_11() {
        assert_eq!(ex_2_11_answer(), (-18.0, 30.0));
    }

    proptest! {
        /// Exercise 2.11: the sign-tested `mul_interval_fast` agrees with
        /// the naive four-product `mul_interval` on every interval pair,
        /// across all nine sign combinations proptest's shrinking finds.
        #[test]
        fn mul_interval_fast_matches_naive(
            (xl, xu) in (-50.0f64..50.0, -50.0f64..50.0)
                .prop_map(|(a, b)| if a <= b { (a, b) } else { (b, a) }),
            (yl, yu) in (-50.0f64..50.0, -50.0f64..50.0)
                .prop_map(|(a, b)| if a <= b { (a, b) } else { (b, a) }),
        ) {
            let x = Interval::new(xl, xu).expect("xl <= xu by construction above");
            let y = Interval::new(yl, yu).expect("yl <= yu by construction above");
            let fast = mul_interval_fast(&x, &y);
            let naive = mul_interval(&x, &y);
            prop_assert!((fast.lower_bound() - naive.lower_bound()).abs() < 1e-9);
            prop_assert!((fast.upper_bound() - naive.upper_bound()).abs() < 1e-9);
        }
    }
}
