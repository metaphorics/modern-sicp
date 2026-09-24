// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercise 2.59 and the edition's addition
//! 2.59a, one module each plus tests.

/// `union-set` for the unordered-list representation: recurses on
/// `set1`, keeping an element only when it is not already in `set2`,
/// and ending in `set2` itself.
fn union_set(set1: &[i128], set2: &[i128]) -> Vec<i128> {
    match set1.split_first() {
        None => set2.to_vec(),
        Some((first, rest)) if set2.contains(first) => union_set(rest, set2),
        Some((first, rest)) => {
            let mut out = vec![*first];
            out.extend(union_set(rest, set2));
            out
        }
    }
}

mod ex_2_59 {
    use super::union_set;

    /// Exercise 2.59: `union-set` for the unordered-list representation
    ///
    /// Returns `union_set(&[1, 2, 3], &[2, 3, 4])`.
    pub fn ex_2_59() -> Vec<i128> {
        union_set(&[1, 2, 3], &[2, 3, 4])
    }
}

#[test]
fn ex_2_59() {
    assert_eq!(ex_2_59::ex_2_59(), vec![1, 2, 3, 4]);
}

mod ex_2_59a {
    use super::union_set;
    use std::collections::HashSet;

    /// Exercise 2.59a (this edition): `union-set` agrees with
    /// `std::collections::HashSet`'s union
    ///
    /// Returns whether `union_set` and `HashSet` union agree, as sets
    /// (ignoring order and the list representation's lack of a
    /// canonical form), on one probe pair.
    pub fn ex_2_59a() -> bool {
        let a = [1_i128, 2, 3];
        let b = [3_i128, 4, 5];
        let ours: HashSet<i128> = union_set(&a, &b).into_iter().collect();
        let std_result: HashSet<i128> = a.iter().chain(b.iter()).copied().collect();
        ours == std_result
    }
}

#[test]
fn ex_2_59a() {
    assert!(ex_2_59a::ex_2_59a());
}

#[cfg(test)]
mod property {
    use super::union_set;
    use proptest::prelude::*;
    use std::collections::HashSet;

    fn dedup_set(mut xs: Vec<i128>) -> Vec<i128> {
        xs.sort_unstable();
        xs.dedup();
        xs
    }

    proptest! {
        /// Exercise 2.59a (this edition): for any two no-duplicate
        /// integer sets, `union_set` and `HashSet` union agree as
        /// sets, regardless of input order or size.
        #[test]
        fn union_set_matches_std_hashset(
            a in proptest::collection::vec(-50i128..50, 0..20),
            b in proptest::collection::vec(-50i128..50, 0..20),
        ) {
            let a = dedup_set(a);
            let b = dedup_set(b);
            let ours: HashSet<i128> = union_set(&a, &b).into_iter().collect();
            let std_result: HashSet<i128> = a.iter().chain(b.iter()).copied().collect();
            prop_assert_eq!(ours, std_result);
        }
    }
}
