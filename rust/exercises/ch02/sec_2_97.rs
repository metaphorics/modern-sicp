// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.97: reducing rational functions. `reduce_terms` and `reduce_poly` behind a generic `reduce`, with the rational package calling it in `make_rat`.

use sicp_runtime::Pending;

mod ex_2_97 {
    use sicp_runtime::Pending;

    /// Exercise 2.97: reducing rational functions. `reduce_terms` and `reduce_poly` behind a generic `reduce`, with the rational package calling it in `make_rat`.
    pub fn ex_2_97() -> Result<(String, String), Pending> {
        Err(Pending::new("2.97"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_97() {
    assert_eq!(ex_2_97::ex_2_97(), Err(Pending::new("2.97")));
}
