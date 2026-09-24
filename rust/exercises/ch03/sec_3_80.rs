// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.80: the series RLC circuit of
//! Figure 3.37, two coupled integrals producing the `v_C` and `i_L`
//! state streams of the network.

mod ex_3_80 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.80`.
        pub exercise: &'static str,
    }

    /// Exercise 3.80: RLC coupled streams
    ///
    /// The solved entry point answers the book's circuit run (`R` = 1,
    /// `L` = 1, `C` = 0.2, `dt` = 0.1, `v_C0` = 10, `i_L0` = 0): the
    /// first six samples each of the `v_C` and `i_L` streams, in
    /// seconds 0.0 through 0.5. The pending body reports [`Pending`].
    pub fn ex_3_80() -> Result<([f64; 6], [f64; 6]), Pending> {
        Err(Pending { exercise: "3.80" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_80() {
    // Hand-checkable Euler steps: v_C[1] = 10 because i_L[0] = 0;
    // i_L[1] = 0 + 0.1 * 10/1 = 1 from v_C[0]; and so on. The pair
    // opens at the stated initial state (v_C(0) = 10, i_L(0) = 0).
    assert_eq!(
        ex_3_80::ex_3_80(),
        Ok((
            [10.0, 10.0, 9.5, 8.55, 7.22, 5.595_5],
            [0.0, 1.0, 1.9, 2.66, 3.249, 3.646_1]
        ))
    );
}
