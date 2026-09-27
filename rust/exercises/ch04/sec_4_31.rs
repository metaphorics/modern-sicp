// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.31: lazy and memo parameter declarations.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_31 {
    //! Exercise 4.31: lazy and memo parameter declarations.

    use super::Pending;

    /// Answers the declared-parameter session values, including the counting probes.
    pub fn ex_4_31() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.31" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_31() {
    let lines = ex_4_31::ex_4_31().expect("solved");
    assert!(lines.contains(&"taken".to_owned()));
    assert!(lines.contains(&"(1 5 5 4 30 30)".to_owned()));
}
