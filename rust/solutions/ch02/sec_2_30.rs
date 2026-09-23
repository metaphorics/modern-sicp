// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.30, one module and one test.

mod ex_2_30 {
    use ch02::sec_2_2::{Nest, leaf, sub};
    use std::rc::Rc;

    /// Exercise 2.30: `square-tree`, the direct definition: the shape
    /// of `count-leaves` with a multiplication at the leaf, recursing
    /// on every child of a branch.
    fn square_tree_direct(tree: &Nest<i128>) -> Nest<i128> {
        match tree {
            Nest::Leaf(x) => leaf(x * x),
            Nest::Sub(items) => Nest::Sub(Rc::new(items.iter().map(square_tree_direct).collect())),
        }
    }

    /// Exercise 2.30: `square-tree`, the `map`-and-recursion definition:
    /// the branch is regarded as a sequence of sub-trees, the map
    /// handles the traversal, and only a mapped element that is itself
    /// a tree recurses.
    fn square_tree_map(tree: &Nest<i128>) -> Nest<i128> {
        match tree {
            Nest::Leaf(x) => leaf(x * x),
            Nest::Sub(items) => Nest::Sub(Rc::new(
                items
                    .iter()
                    .map(|sub_tree| match sub_tree {
                        Nest::Sub(_) => square_tree_map(sub_tree),
                        Nest::Leaf(x) => leaf(x * x),
                    })
                    .collect(),
            )),
        }
    }

    /// Exercise 2.30: square-tree
    ///
    /// Returns the rendered square of the tree `(1 (2 (3 4) 5) (6 7))`
    /// by the direct recursion and by the `map`-and-recursion version,
    /// in that order.
    pub fn ex_2_30() -> (String, String) {
        let tree = sub(&[
            leaf(1),
            sub(&[leaf(2), sub(&[leaf(3), leaf(4)]), leaf(5)]),
            sub(&[leaf(6), leaf(7)]),
        ]);
        let direct = square_tree_direct(&tree);
        let mapped = square_tree_map(&tree);
        (direct.to_string(), mapped.to_string())
    }
}

#[test]
fn ex_2_30() {
    assert_eq!(
        ex_2_30::ex_2_30(),
        (
            "(1 (4 (9 16) 25) (36 49))".to_string(),
            "(1 (4 (9 16) 25) (36 49))".to_string()
        )
    );
}
