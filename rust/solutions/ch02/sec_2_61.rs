// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.61, one module and one test.

mod ex_2_61 {
    /// `adjoin-set` for the ordered-list representation: scans past
    /// elements smaller than `x`, taking advantage of the ordering the
    /// way `element-of-set?` does, and stops once an element is not
    /// smaller than `x` (returning the set unchanged if it is equal).
    /// On average about half as many steps as the unordered
    /// representation's `Θ(n)` adjoin. The scan and the rebuild are
    /// each one pass, so the whole call is `Θ(n)`.
    fn adjoin_set_ordered(x: i128, set: &[i128]) -> Vec<i128> {
        let mut out = Vec::with_capacity(set.len() + 1);
        for (i, &y) in set.iter().enumerate() {
            match x.cmp(&y) {
                std::cmp::Ordering::Equal => {
                    out.extend_from_slice(&set[i..]);
                    return out;
                }
                std::cmp::Ordering::Less => {
                    out.push(x);
                    out.extend_from_slice(&set[i..]);
                    return out;
                }
                std::cmp::Ordering::Greater => out.push(y),
            }
        }
        out.push(x);
        out
    }

    /// Exercise 2.61: `adjoin-set` for the ordered-list representation
    ///
    /// Returns `adjoin_set_ordered(5, &[1, 3, 6, 10])` (inserting a
    /// new element) and `adjoin_set_ordered(3, &[1, 3, 6, 10])`
    /// (adjoining an already-present element), in that order.
    pub fn ex_2_61() -> (Vec<i128>, Vec<i128>) {
        (
            adjoin_set_ordered(5, &[1, 3, 6, 10]),
            adjoin_set_ordered(3, &[1, 3, 6, 10]),
        )
    }
}

#[test]
fn ex_2_61() {
    assert_eq!(
        ex_2_61::ex_2_61(),
        (vec![1, 3, 5, 6, 10], vec![1, 3, 6, 10])
    );
}
