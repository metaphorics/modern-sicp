// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.64, one module and one test.

mod ex_2_64 {
    use ch02::sec_2_3::Tree;

    /// Converts an ordered list of `n` elements into a balanced binary
    /// tree: the book's `list->tree`.
    fn list_to_tree(elements: &[i128]) -> Tree {
        let mut nodes_built = 0;
        partial_tree(elements, elements.len(), &mut nodes_built).0
    }

    /// Builds a balanced tree over the first `n` elements of `elts`,
    /// returning that tree and the elements not included in it: the
    /// book's `partial-tree`, returning `(Tree, &[i128])` where the
    /// book returns a `cons` pair. Splitting the count in half at each
    /// level (`left_size`, then `right_size` from what remains after
    /// placing the middle element as the entry) is what keeps the
    /// tree balanced: the left subtree gets about half of `n`, the
    /// entry takes one, and the right subtree gets the rest.
    fn partial_tree<'a>(elts: &'a [i128], n: usize, nodes_built: &mut u64) -> (Tree, &'a [i128]) {
        if n == 0 {
            return (Tree::Empty, elts);
        }
        let left_size = (n - 1) / 2;
        let (left_tree, non_left_elts) = partial_tree(elts, left_size, nodes_built);
        let right_size = n - (left_size + 1);
        let (this_entry, after_entry) = non_left_elts
            .split_first()
            .expect("partial_tree: n elements remain after the left subtree");
        let (right_tree, remaining_elts) = partial_tree(after_entry, right_size, nodes_built);
        *nodes_built += 1;
        (
            Tree::make_tree(*this_entry, left_tree, right_tree),
            remaining_elts,
        )
    }

    fn node_count(n: usize) -> u64 {
        let elements: Vec<i128> = (1..=n as i128).collect();
        let mut nodes_built = 0;
        partial_tree(&elements, n, &mut nodes_built);
        nodes_built
    }

    /// Exercise 2.64: `list->tree`
    ///
    /// Part a: `partial_tree` places the middle element as the
    /// current node's entry, builds the left subtree from everything
    /// before it and the right subtree from everything after, and
    /// returns the constructed node paired with whatever list
    /// elements were not yet placed (empty here, since the top call
    /// consumes the whole list). For `[1, 3, 5, 7, 9, 11]`: entry `5`
    /// (the middle), left subtree over `[1, 3]` (entry `1`, right
    /// child `3`), right subtree over `[7, 9, 11]` (entry `9`,
    /// children `7` and `11`).
    ///
    /// Part b: every element becomes exactly one tree node (one
    /// `make_tree` call each, no element visited twice), so the node
    /// count equals the input length exactly at both 11 and 101
    /// elements: `list->tree` is `Θ(n)`.
    ///
    /// Returns the printed tree for `[1, 3, 5, 7, 9, 11]`, and the
    /// node counts for 11 and 101 elements, in that order.
    pub fn ex_2_64() -> (String, u64, u64) {
        let tree = list_to_tree(&[1, 3, 5, 7, 9, 11]);
        (tree.to_string(), node_count(11), node_count(101))
    }
}

#[test]
fn ex_2_64() {
    assert_eq!(
        ex_2_64::ex_2_64(),
        (
            "(5 (1 () (3 () ())) (9 (7 () ()) (11 () ())))".to_string(),
            11,
            101
        )
    );
}
