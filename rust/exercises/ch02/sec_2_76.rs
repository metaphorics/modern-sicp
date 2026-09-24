// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.76, one module and one ignored
//! test.

mod ex_2_76 {
    use sicp_runtime::Pending;

    /// Exercise 2.76: adding types versus adding operations
    ///
    /// Serves the real part and the magnitude of the same complex number
    /// through three organizations of a two-representation system:
    /// explicit dispatch on an enum, the operation table, and
    /// message-passing objects. Returns the `(real_part, magnitude)` pair
    /// each organization answers.
    #[allow(
        clippy::type_complexity,
        reason = "one probe pair per strategy, matching the exercise's three-way comparison"
    )]
    pub fn ex_2_76() -> Result<((f64, f64), (f64, f64), (f64, f64)), Pending> {
        Err(Pending::new("2.76"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_76() {
    assert_eq!(ex_2_76::ex_2_76(), Ok(((3.0, 5.0), (3.0, 5.0), (3.0, 5.0))));
}
