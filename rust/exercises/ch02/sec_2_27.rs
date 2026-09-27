// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.27, one module and one ignored
//! test.

mod ex_2_27 {
    use sicp_runtime::Pending;

    /// Exercise 2.27: deep-reverse
    ///
    /// Returns the rendered `deep_reverse` of `x = ((1 2) (3 4))`, with the
    /// sublists reversed too.
    pub fn ex_2_27() -> Result<String, Pending> {
        Err(Pending::new("2.27"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_27() {
    assert_eq!(ex_2_27::ex_2_27(), Ok("((4 3) (2 1))".to_string()));
}
