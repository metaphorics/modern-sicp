// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.65, one module and one test.

mod ex_2_65 {
    use ch02::sec_2_3::{Tree, intersection_set_ordered};
    use std::collections::VecDeque;

    /// In-order traversal with an accumulator, exactly exercise
    /// 2.63's `Θ(n)` `tree_to_list_2`.
    fn tree_to_list(tree: &Tree) -> Vec<i128> {
        fn copy_to_list(tree: &Tree, mut result: VecDeque<i128>) -> VecDeque<i128> {
            match tree {
                Tree::Empty => result,
                Tree::Node(..) => {
                    result = copy_to_list(tree.right_branch().expect("node"), result);
                    result.push_front(tree.entry().expect("node"));
                    copy_to_list(tree.left_branch().expect("node"), result)
                }
            }
        }
        copy_to_list(tree, VecDeque::new()).into_iter().collect()
    }

    /// Exercise 2.64's `Θ(n)` `list_to_tree`.
    fn list_to_tree(elements: &[i128]) -> Tree {
        partial_tree(elements, elements.len()).0
    }

    fn partial_tree(elts: &[i128], n: usize) -> (Tree, &[i128]) {
        if n == 0 {
            return (Tree::Empty, elts);
        }
        let left_size = (n - 1) / 2;
        let (left_tree, non_left_elts) = partial_tree(elts, left_size);
        let right_size = n - (left_size + 1);
        let (this_entry, after_entry) = non_left_elts.split_first().expect("n elements remain");
        let (right_tree, remaining_elts) = partial_tree(after_entry, right_size);
        (
            Tree::make_tree(*this_entry, left_tree, right_tree),
            remaining_elts,
        )
    }

    /// Exercise 2.62's `Θ(n)` `union_set_ordered`.
    fn union_set_ordered(set1: &[i128], set2: &[i128]) -> Vec<i128> {
        let mut out = Vec::with_capacity(set1.len() + set2.len());
        let mut i = 0;
        let mut j = 0;
        while i < set1.len() && j < set2.len() {
            match set1[i].cmp(&set2[j]) {
                std::cmp::Ordering::Equal => {
                    out.push(set1[i]);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Less => {
                    out.push(set1[i]);
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    out.push(set2[j]);
                    j += 1;
                }
            }
        }
        out.extend_from_slice(&set1[i..]);
        out.extend_from_slice(&set2[j..]);
        out
    }

    /// Θ(n) `union-set` for tree sets: flatten both trees to their
    /// (already sorted) element lists, merge in `Θ(n)` as ordered
    /// lists, and rebuild a balanced tree from the merged list.
    /// Flatten, merge, and rebuild are each `Θ(n)`, so the whole
    /// pipeline is `Θ(n)`.
    fn union_set_tree(t1: &Tree, t2: &Tree) -> Tree {
        list_to_tree(&union_set_ordered(&tree_to_list(t1), &tree_to_list(t2)))
    }

    /// Θ(n) `intersection-set` for tree sets, the same pipeline as
    /// `union_set_tree` with the library's `intersection_set_ordered`
    /// in place of the merge step.
    fn intersection_set_tree(t1: &Tree, t2: &Tree) -> Tree {
        list_to_tree(&intersection_set_ordered(
            &tree_to_list(t1),
            &tree_to_list(t2),
        ))
    }

    /// Exercise 2.65: Θ(n) `union-set` and `intersection-set` for tree
    /// sets
    ///
    /// Returns the printed union and the printed intersection of the
    /// tree built from `[1, 3, 5]` and the tree built from
    /// `[3, 5, 7]`, in that order.
    pub fn ex_2_65() -> (String, String) {
        let t1 = list_to_tree(&[1, 3, 5]);
        let t2 = list_to_tree(&[3, 5, 7]);
        (
            union_set_tree(&t1, &t2).to_string(),
            intersection_set_tree(&t1, &t2).to_string(),
        )
    }
}

#[test]
fn ex_2_65() {
    assert_eq!(
        ex_2_65::ex_2_65(),
        (
            "(3 (1 () ()) (5 () (7 () ())))".to_string(),
            "(3 () (5 () ()))".to_string(),
        )
    );
}
