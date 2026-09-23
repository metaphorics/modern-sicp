// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.32, one module and one ignored
//! test.

mod ex_2_32 {
    use sicp_runtime::Pending;

    /// Exercise 2.32: subsets
    ///
    /// Returns the rendered set of all subsets of `(1 2 3)`.
    pub fn ex_2_32() -> Result<String, Pending> {
        Err(Pending::new("2.32"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_32() {
    assert_eq!(
        ex_2_32::ex_2_32(),
        Ok("(() (3) (2) (2 3) (1) (1 3) (1 2) (1 2 3))".to_string())
    );
}
