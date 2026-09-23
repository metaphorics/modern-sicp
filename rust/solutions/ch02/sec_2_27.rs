// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.27, one module and one test.

mod ex_2_27 {
    use ch02::sec_2_2::{List, Nest, leaf, sub};
    use std::rc::Rc;

    /// Exercise 2.27: `deep-reverse`
    ///
    /// Where exercise 2.18's `reverse` turned one list around,
    /// `deep_reverse` recurses: each branch of a tree is rebuilt with
    /// its children in the opposite order, each child itself
    /// deep-reversed. Leaves are their own answer, which is the base
    /// case Scheme's `pair?` test found.
    fn deep_reverse(tree: &Nest<i128>) -> Nest<i128> {
        match tree {
            Nest::Leaf(x) => Nest::Leaf(*x),
            Nest::Sub(items) => {
                let mut reversed = List::Nil;
                for child in items.iter() {
                    reversed = List::cons(deep_reverse(child), &reversed);
                }
                Nest::Sub(Rc::new(reversed))
            }
        }
    }

    /// Exercise 2.27: deep-reverse
    ///
    /// Returns the rendered `deep_reverse` of `x = ((1 2) (3 4))`, with
    /// the sublists reversed too.
    pub fn ex_2_27() -> String {
        let x = sub(&[sub(&[leaf(1), leaf(2)]), sub(&[leaf(3), leaf(4)])]);
        deep_reverse(&x).to_string()
    }
}

#[test]
fn ex_2_27() {
    assert_eq!(ex_2_27::ex_2_27(), "((4 3) (2 1))".to_string());
}
