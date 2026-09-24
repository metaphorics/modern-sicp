// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.54, one module and one test.

mod ex_2_54 {
    use ch02::sec_2_2::{List, Nest, leaf, sub};
    use sicp_runtime::Symbol;

    /// Recursive `equal?` over symbol-leaf trees: two leaves are equal
    /// when their symbols are `eq?` (content equal, since `Symbol` is
    /// `Rc<str>`); two branches are equal when they hold the same
    /// number of items and each pair of items is recursively equal, in
    /// order; a leaf and a branch are never equal. `Nest` already
    /// derives structural `PartialEq`, but the derive is not this
    /// exercise's contribution: writing the recursion by hand is.
    fn my_equal(a: &Nest<Symbol>, b: &Nest<Symbol>) -> bool {
        match (a, b) {
            (Nest::Leaf(x), Nest::Leaf(y)) => x == y,
            (Nest::Sub(xs), Nest::Sub(ys)) => equal_lists(xs, ys),
            _ => false,
        }
    }

    fn equal_lists(xs: &List<Nest<Symbol>>, ys: &List<Nest<Symbol>>) -> bool {
        match (xs, ys) {
            (List::Nil, List::Nil) => true,
            (List::Cons(x, xrest), List::Cons(y, yrest)) => {
                my_equal(x, y) && equal_lists(xrest, yrest)
            }
            _ => false,
        }
    }

    /// Exercise 2.54: `equal?`
    ///
    /// Returns whether `(this is a list)` is equal to itself, and
    /// whether it is equal to `(this (is a) list)`, in that order.
    pub fn ex_2_54() -> (bool, bool) {
        let flat = sub(&[
            leaf(Symbol::from("this")),
            leaf(Symbol::from("is")),
            leaf(Symbol::from("a")),
            leaf(Symbol::from("list")),
        ]);
        let nested = sub(&[
            leaf(Symbol::from("this")),
            sub(&[leaf(Symbol::from("is")), leaf(Symbol::from("a"))]),
            leaf(Symbol::from("list")),
        ]);
        (my_equal(&flat, &flat), my_equal(&flat, &nested))
    }
}

#[test]
fn ex_2_54() {
    assert_eq!(ex_2_54::ex_2_54(), (true, false));
}
