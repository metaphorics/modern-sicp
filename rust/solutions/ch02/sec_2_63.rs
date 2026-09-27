// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.63, one module and one test.

mod ex_2_63 {
    use ch02::sec_2_3::Tree;
    use std::collections::VecDeque;

    /// Converts a binary tree to a list by appending the left
    /// subtree's list, the entry, and the right subtree's list: the
    /// book's `tree->list-1`. Every `extend` copies the right
    /// subtree's whole result, so on a balanced tree this is
    /// `Θ(n log n)`.
    fn tree_to_list_1_copies(tree: &Tree, copies: &mut u64) -> Vec<i128> {
        match tree {
            Tree::Empty => Vec::new(),
            Tree::Node(..) => {
                let entry = tree.entry().expect("node");
                let mut out = tree_to_list_1_copies(
                    tree.left_branch().expect("node has a left branch"),
                    copies,
                );
                out.push(entry);
                let right_list = tree_to_list_1_copies(
                    tree.right_branch().expect("node has a right branch"),
                    copies,
                );
                *copies += right_list.len() as u64;
                out.extend(right_list);
                out
            }
        }
    }

    /// Converts a binary tree to a list with an accumulator, `cons`ing
    /// one entry at a time and never copying an already-built result:
    /// the book's `tree->list-2`. Genuinely `Θ(n)`, one accumulator
    /// step per node.
    fn tree_to_list_2_copies(tree: &Tree, copies: &mut u64) -> Vec<i128> {
        fn copy_to_list(
            tree: &Tree,
            mut result: VecDeque<i128>,
            copies: &mut u64,
        ) -> VecDeque<i128> {
            match tree {
                Tree::Empty => result,
                Tree::Node(..) => {
                    result = copy_to_list(
                        tree.right_branch().expect("node has a right branch"),
                        result,
                        copies,
                    );
                    result.push_front(tree.entry().expect("node"));
                    *copies += 1;
                    copy_to_list(
                        tree.left_branch().expect("node has a left branch"),
                        result,
                        copies,
                    )
                }
            }
        }
        copy_to_list(tree, VecDeque::new(), copies)
            .into_iter()
            .collect()
    }

    /// A balanced tree over a sorted slice, splitting on the middle
    /// element at every level: a private test fixture, independent of
    /// exercise 2.64's `list->tree`.
    fn balanced_tree(elements: &[i128]) -> Tree {
        if elements.is_empty() {
            return Tree::Empty;
        }
        let mid = elements.len() / 2;
        Tree::make_tree(
            elements[mid],
            balanced_tree(&elements[..mid]),
            balanced_tree(&elements[mid + 1..]),
        )
    }

    fn copies_at(n: i128) -> (u64, u64) {
        let elements: Vec<i128> = (1..=n).collect();
        let tree = balanced_tree(&elements);
        let mut c1 = 0;
        let mut c2 = 0;
        assert_eq!(tree_to_list_1_copies(&tree, &mut c1), elements);
        assert_eq!(tree_to_list_2_copies(&tree, &mut c2), elements);
        (c1, c2)
    }

    /// Exercise 2.63: comparing `tree_to_list_1` and `tree_to_list_2`
    ///
    /// Part a: whether the three trees of Figure 2.16 agree under
    /// `tree_to_list_1`, and whether `tree_to_list_2` agrees with it
    /// on all three: both `true`, since in-order traversal always
    /// produces the sorted element list regardless of which of the
    /// tree's several valid shapes represents the set. Part b: the
    /// list-cell copy counts of both procedures on balanced trees of
    /// 15 and 127 elements. `tree_to_list_2`'s count equals the
    /// element count exactly (one `cons` per node); `tree_to_list_1`'s
    /// grows faster, from about 1.1x at 15 elements to about 2.5x at
    /// 127, the `Θ(n log n)` versus `Θ(n)` gap widening with `n`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's two-part statement"
    )]
    pub fn ex_2_63() -> (bool, bool, (u64, u64, u64, u64)) {
        // Figure 2.16's three trees, all representing {1, 3, 5, 7, 9, 11}.
        let tree_a = Tree::make_tree(
            7,
            Tree::make_tree(
                3,
                Tree::make_tree(1, Tree::Empty, Tree::Empty),
                Tree::make_tree(5, Tree::Empty, Tree::Empty),
            ),
            Tree::make_tree(
                9,
                Tree::Empty,
                Tree::make_tree(11, Tree::Empty, Tree::Empty),
            ),
        );
        let tree_b = Tree::make_tree(
            3,
            Tree::make_tree(1, Tree::Empty, Tree::Empty),
            Tree::make_tree(
                7,
                Tree::make_tree(5, Tree::Empty, Tree::Empty),
                Tree::make_tree(
                    9,
                    Tree::Empty,
                    Tree::make_tree(11, Tree::Empty, Tree::Empty),
                ),
            ),
        );
        let tree_c = Tree::make_tree(
            5,
            Tree::make_tree(3, Tree::make_tree(1, Tree::Empty, Tree::Empty), Tree::Empty),
            Tree::make_tree(
                9,
                Tree::make_tree(7, Tree::Empty, Tree::Empty),
                Tree::make_tree(11, Tree::Empty, Tree::Empty),
            ),
        );

        let expected = vec![1, 3, 5, 7, 9, 11];
        let mut unused = 0;
        let results_1: Vec<Vec<i128>> = [&tree_a, &tree_b, &tree_c]
            .iter()
            .map(|t| tree_to_list_1_copies(t, &mut unused))
            .collect();
        let same_for_every_tree_1 = results_1.iter().all(|r| r == &expected);
        let results_2: Vec<Vec<i128>> = [&tree_a, &tree_b, &tree_c]
            .iter()
            .map(|t| tree_to_list_2_copies(t, &mut unused))
            .collect();
        let tree_to_list_2_agrees = results_1 == results_2;

        let (c1_small, c2_small) = copies_at(15);
        let (c1_large, c2_large) = copies_at(127);

        (
            same_for_every_tree_1,
            tree_to_list_2_agrees,
            (c1_small, c2_small, c1_large, c2_large),
        )
    }
}

#[test]
fn ex_2_63() {
    assert_eq!(ex_2_63::ex_2_63(), (true, true, (17, 15, 321, 127)));
}
