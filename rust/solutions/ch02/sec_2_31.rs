// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.31, one module and one test.

mod ex_2_31 {
    use ch02::sec_2_2::{Nest, leaf, sub};
    use std::rc::Rc;

    /// Exercise 2.31: `tree-map`
    ///
    /// The abstracted `square-tree`: the leaf operation arrives as a
    /// function, the traversal stays the same recursion over the tree's
    /// children. The operation parameter is `Copy` so the recursive
    /// call can carry it down; the squaring closure captures nothing.
    fn tree_map(f: impl Fn(i128) -> i128 + Copy, tree: &Nest<i128>) -> Nest<i128> {
        match tree {
            Nest::Leaf(x) => leaf(f(*x)),
            Nest::Sub(items) => Nest::Sub(Rc::new(items.iter().map(|t| tree_map(f, t)).collect())),
        }
    }

    /// Exercise 2.31: `square-tree` defined as a `tree-map` call, the
    /// property the exercise asks the abstraction to have.
    fn square_tree(tree: &Nest<i128>) -> Nest<i128> {
        tree_map(|x| x * x, tree)
    }

    /// Exercise 2.31: tree-map
    ///
    /// Returns the rendered square of `(1 (2 (3 4) 5) (6 7))` produced
    /// by `square_tree` defined as a `tree_map` call.
    pub fn ex_2_31() -> String {
        let tree = sub(&[
            leaf(1),
            sub(&[leaf(2), sub(&[leaf(3), leaf(4)]), leaf(5)]),
            sub(&[leaf(6), leaf(7)]),
        ]);
        square_tree(&tree).to_string()
    }
}

#[test]
fn ex_2_31() {
    assert_eq!(ex_2_31::ex_2_31(), "(1 (4 (9 16) 25) (36 49))".to_string());
}
