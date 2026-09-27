// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.28, one module and one test.

mod ex_2_28 {
    use ch02::sec_2_2::{List, Nest, append, leaf, sub};

    /// Exercise 2.28: `fringe`
    ///
    /// A leaf contributes itself as the only element of a one-element
    /// list; a branch contributes the concatenation of its children's
    /// fringes, left to right. The result is the flat list of leaves,
    /// which 2.2.3 renames `enumerate-tree` once the tree stops being
    /// the object of interest and becomes one more sequence source.
    fn fringe(tree: &Nest<i128>) -> List<i128> {
        match tree {
            Nest::Leaf(x) => List::cons(*x, &List::Nil),
            Nest::Sub(items) => {
                let mut acc = List::Nil;
                for child in items.iter() {
                    acc = append(&acc, &fringe(child));
                }
                acc
            }
        }
    }

    /// Exercise 2.28: fringe
    ///
    /// Returns the rendered leaves of `x = ((1 2) (3 4))` and of
    /// `list(x, x)`, in that order.
    pub fn ex_2_28() -> (String, String) {
        let x = sub(&[sub(&[leaf(1), leaf(2)]), sub(&[leaf(3), leaf(4)])]);
        let doubled = sub(&[x.clone(), x.clone()]);
        (fringe(&x).to_string(), fringe(&doubled).to_string())
    }
}

#[test]
fn ex_2_28() {
    assert_eq!(
        ex_2_28::ex_2_28(),
        ("(1 2 3 4)".to_string(), "(1 2 3 4 1 2 3 4)".to_string())
    );
}
