// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.35, one module and one test.

mod ex_2_35 {
    use ch02::sec_2_2::{Nest, accumulate, enumerate_tree, leaf, sub};

    /// Exercise 2.35: `count-leaves` as an accumulation
    ///
    /// The book's fill for `(accumulate ⟨??⟩ ⟨??⟩ (map ⟨??⟩ ⟨??⟩))`:
    /// enumerate the leaves of the tree (2.28's `fringe`, this
    /// edition's `enumerate_tree`), map a constant one over every
    /// leaf, and accumulate with `+` from 0. The tree recursion of
    /// 2.2.2's direct definition becomes three signal-flow stages.
    fn count_leaves(tree: &Nest<i128>) -> u64 {
        let leaves: Vec<i128> = enumerate_tree(tree).iter().copied().collect();
        accumulate(|_, n| n + 1, 0, &leaves)
    }

    /// Exercise 2.35: count-leaves as an accumulation
    ///
    /// Returns the leaf count of `list(x, x)` where `x = ((1 2) (3 4))`,
    /// computed as an accumulation over the enumerated leaves.
    pub fn ex_2_35() -> u64 {
        let x = sub(&[sub(&[leaf(1), leaf(2)]), sub(&[leaf(3), leaf(4)])]);
        count_leaves(&sub(&[x.clone(), x]))
    }
}

#[test]
fn ex_2_35() {
    assert_eq!(ex_2_35::ex_2_35(), 8);
}
