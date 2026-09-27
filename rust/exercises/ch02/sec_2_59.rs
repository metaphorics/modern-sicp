// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 2.59 and the edition's addition
//! 2.59a, one module and one ignored test per exercise.

mod ex_2_59 {
    use sicp_runtime::Pending;

    /// Exercise 2.59: `union-set` for the unordered-list representation
    ///
    /// Returns `union_set(&[1, 2, 3], &[2, 3, 4])`.
    pub fn ex_2_59() -> Result<Vec<i128>, Pending> {
        Err(Pending::new("2.59"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_59() {
    assert_eq!(ex_2_59::ex_2_59(), Ok(vec![1, 2, 3, 4]));
}

mod ex_2_59a {
    use sicp_runtime::Pending;

    /// Exercise 2.59a (this edition): `union-set` agrees with
    /// `std::collections::HashSet`'s union
    ///
    /// Returns whether `union_set` and `HashSet` union agree, as sets,
    /// on one probe pair.
    pub fn ex_2_59a() -> Result<bool, Pending> {
        Err(Pending::new("2.59a"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_59a() {
    assert_eq!(ex_2_59a::ex_2_59a(), Ok(true));
}
