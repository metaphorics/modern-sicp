// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.77: the direct
//! `integers-starting-from`-style integral rewritten to take a delayed
//! integrand, exercised on a finite integrand and in the solve loop for
//! dy/dt = y.

mod ex_3_77 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.77`.
        pub exercise: &'static str,
    }

    /// Exercise 3.77: integral with delayed integrand
    ///
    /// Answers the direct integral of the integers 0..=9 at dt = 1 from
    /// initial value 0, the number of elements a full walk of that
    /// finite integral visits, element 1000 of the direct solve of
    /// dy/dt = y with y0 = 1 at dt = 0.001, and the same element from
    /// the section's own solve.
    pub fn ex_3_77() -> Result<([f64; 11], usize, f64, f64), Pending> {
        Err(Pending { exercise: "3.77" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_77() {
    let (sums, walked, direct, module) = ex_3_77::ex_3_77().expect("solved");
    // Integrating the integers 0..=9 at dt = 1 answers the triangular
    // totals k(k-1)/2, and the stream ends after 11 elements.
    for (k, got) in sums.iter().enumerate() {
        let k = i128::try_from(k).expect("index fits i128");
        #[expect(
            clippy::cast_precision_loss,
            reason = "the triangular totals stay exact in f64 at this range"
        )]
        let want = (k * (k - 1) / 2) as f64;
        assert!((got - want).abs() < 1e-12, "sum {k}: {got} vs {want}");
    }
    assert_eq!(walked, 11);
    // The solve loop works on the delayed integrand: element 1000 is
    // (1 + dt)^1000, within 1e-2 of e, and the direct and implicit
    // integrators agree to the last bit.
    assert!(
        (direct - std::f64::consts::E).abs() < 1e-2,
        "direct solve at 1000: {direct}"
    );
    assert!(
        (direct - module).abs() < 1e-12,
        "direct {direct} vs module {module}"
    );
}
