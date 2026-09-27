// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.19: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_19 {
    /// `Fib(n)` by the ordinary linear iteration, used only to check the
    /// squaring version against a trusted reference.
    fn fib_reference(n: u64) -> i128 {
        let (mut a, mut b, mut count) = (1_i128, 0_i128, n);
        while count != 0 {
            (a, b) = (a + b, a);
            count -= 1;
        }
        b
    }

    /// `Fib(n)` by squaring the transformation `T`. Applying `T_pq`
    /// twice is again a `T_p'q'`, with `p' = p^2 + q^2` and
    /// `q' = q^2 + 2pq`: squaring `p` and `q` this way is what lets the
    /// exponent halve every step, the same shape as `fast-expt`.
    fn fib_squaring(n: u64) -> i128 {
        let (mut fib_a, mut fib_b, mut trans_p, mut trans_q, mut count) =
            (1_i128, 0_i128, 0_i128, 1_i128, n);
        while count != 0 {
            if count.is_multiple_of(2) {
                let p_next = trans_p * trans_p + trans_q * trans_q;
                let q_next = trans_q * trans_q + 2 * trans_p * trans_q;
                trans_p = p_next;
                trans_q = q_next;
                count /= 2;
            } else {
                let a_next = fib_b * trans_q + fib_a * trans_q + fib_a * trans_p;
                let b_next = fib_b * trans_p + fib_a * trans_q;
                fib_a = a_next;
                fib_b = b_next;
                count -= 1;
            }
        }
        fib_b
    }

    /// Exercise 1.19: Fibonacci by transformation squaring
    ///
    /// Returns `fib(90)` as the logarithmic process computes it, with the
    /// state carried in `i128` because the pair outgrows 64-bit integers
    /// near `Fib(93)`, together with whether it agrees with the linear
    /// reference for every `n` from 0 through 90.
    pub fn ex_1_19() -> (i128, bool) {
        let agrees = (0..=90).all(|n| fib_squaring(n) == fib_reference(n));
        (fib_squaring(90), agrees)
    }
}

#[test]
fn ex_1_19() {
    let (value, agrees) = ex_1_19::ex_1_19();
    assert!(agrees, "fib_squaring disagrees with the linear reference");
    assert_eq!(value, 2_880_067_194_370_816_120);
}
