// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.73: the RC circuit of Figure
//! 3.33 as a stream processor, the unit-current response's first six
//! voltages for r = 0.5 ohm, c = 1 farad, dt = 0.2 second, v0 = 0.

mod ex_3_73 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.73`.
        pub exercise: &'static str,
    }

    /// Exercise 3.73: RC circuit signal processor
    ///
    /// Answers the first six voltages of the unit-current response for
    /// r = 0.5 ohm, c = 1 farad, dt = 0.2 second, v0 = 0.
    pub fn ex_3_73() -> Result<[f64; 6], Pending> {
        Err(Pending { exercise: "3.73" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_73() {
    let v = ex_3_73::ex_3_73().expect("solved");
    // v[k] = 0.2*k + 0.5: the integrator contributes 0.2*k, the
    // resistor path adds r*i = 0.5 at every index.
    for (got, want) in v.iter().zip([0.5, 0.7, 0.9, 1.1, 1.3, 1.5]) {
        assert!((got - want).abs() < 1e-12, "v: {got} vs {want}");
    }
}
