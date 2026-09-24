// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.62, one module and one test.

mod ex_2_62 {
    use std::cmp::Ordering;

    /// A `Θ(n)` `union-set` for ordered-list sets: walks both lists in
    /// step like `intersection_set_ordered`, taking the smaller head
    /// (or the shared head once, advancing both) at each step, so the
    /// merge visits every element of both lists exactly once and the
    /// result `Vec` is built in one pass.
    fn union_set_ordered(set1: &[i128], set2: &[i128]) -> Vec<i128> {
        let mut out = Vec::with_capacity(set1.len() + set2.len());
        let mut i = 0;
        let mut j = 0;
        while i < set1.len() && j < set2.len() {
            match set1[i].cmp(&set2[j]) {
                Ordering::Equal => {
                    out.push(set1[i]);
                    i += 1;
                    j += 1;
                }
                Ordering::Less => {
                    out.push(set1[i]);
                    i += 1;
                }
                Ordering::Greater => {
                    out.push(set2[j]);
                    j += 1;
                }
            }
        }
        out.extend_from_slice(&set1[i..]);
        out.extend_from_slice(&set2[j..]);
        out
    }

    /// Exercise 2.62: a Θ(n) `union-set` for ordered-list sets
    ///
    /// Returns `union_set_ordered(&[1, 3, 6, 10], &[2, 3, 7])`.
    pub fn ex_2_62() -> Vec<i128> {
        union_set_ordered(&[1, 3, 6, 10], &[2, 3, 7])
    }
}

#[test]
fn ex_2_62() {
    assert_eq!(ex_2_62::ex_2_62(), vec![1, 2, 3, 6, 7, 10]);
}
