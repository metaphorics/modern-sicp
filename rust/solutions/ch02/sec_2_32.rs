// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.32, one module and one test.

mod ex_2_32 {
    use ch02::sec_2_2::{List, append, map_list};

    /// Exercise 2.32: `subsets`
    ///
    /// The completed definition: the subsets of `s` are the subsets of
    /// its `cdr` (those that leave the first element out), together
    /// with the same subsets each extended by a first element at the
    /// front (those that include it). The base case is the one subset
    /// of the empty set, the empty set itself, which is why the empty
    /// branch returns the list containing the empty list. The result
    /// counts `2^n` elements because every element doubles the count.
    fn subsets(s: &List<i128>) -> List<List<i128>> {
        match s {
            List::Nil => List::cons(List::Nil, &List::Nil),
            List::Cons(x, rest) => {
                let rest_subsets = subsets(rest);
                let extended = map_list(|subset| List::cons(*x, subset), &rest_subsets);
                append(&rest_subsets, &extended)
            }
        }
    }

    /// Exercise 2.32: subsets
    ///
    /// Returns the rendered set of all subsets of `(1 2 3)`.
    pub fn ex_2_32() -> String {
        let s = List::from_iter([1, 2, 3]);
        subsets(&s).to_string()
    }
}

#[test]
fn ex_2_32() {
    assert_eq!(
        ex_2_32::ex_2_32(),
        "(() (3) (2) (2 3) (1) (1 3) (1 2) (1 2 3))".to_string()
    );
}
