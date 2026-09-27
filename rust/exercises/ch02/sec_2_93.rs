// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.93: the rational package over generic values, with no reduction in `make_rat`: adding the book's rational function to itself leaves an unreduced product.

use sicp_runtime::Pending;

mod ex_2_93 {
    use sicp_runtime::Pending;

    /// Exercise 2.93: the rational package over generic values, with no reduction in `make_rat`: adding the book's rational function to itself leaves an unreduced product.
    pub fn ex_2_93() -> Result<(String, String), Pending> {
        Err(Pending::new("2.93"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_93() {
    assert_eq!(ex_2_93::ex_2_93(), Err(Pending::new("2.93")));
}
