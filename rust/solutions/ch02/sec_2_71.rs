// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.71, one module and one test.

mod ex_2_71 {
    use ch02::sec_2_3::{HuffmanTree, adjoin_huffman_set, make_leaf_set};
    use sicp_runtime::Symbol;

    fn successive_merge(mut set: Vec<HuffmanTree>) -> Option<HuffmanTree> {
        loop {
            if set.len() <= 1 {
                return set.into_iter().next();
            }
            let left = set.remove(0);
            let right = set.remove(0);
            let merged = HuffmanTree::make_code_tree(left, right);
            set = adjoin_huffman_set(merged, &set);
        }
    }

    fn generate_huffman_tree(pairs: &[(Symbol, u32)]) -> Option<HuffmanTree> {
        successive_merge(make_leaf_set(pairs))
    }

    fn sym(s: &str) -> Symbol {
        Symbol::from(s)
    }

    /// The depth (number of branches from the root) of `target`'s
    /// leaf, or `None` if it is not in `tree`.
    fn depth_of(tree: &HuffmanTree, target: &Symbol, current: u32) -> Option<u32> {
        if tree.is_leaf() {
            return (tree.symbol_leaf() == Some(target)).then_some(current);
        }
        tree.left_branch()
            .and_then(|l| depth_of(l, target, current + 1))
            .or_else(|| {
                tree.right_branch()
                    .and_then(|r| depth_of(r, target, current + 1))
            })
    }

    /// Builds the geometric-frequency alphabet `S0..S(n-1)` with
    /// `S_i` at weight `2^i`, and returns the depth of the most
    /// frequent symbol (`S(n-1)`) and one least frequent symbol
    /// (`S0`).
    fn depths(n: u32) -> (u32, u32) {
        let pairs: Vec<(Symbol, u32)> = (0..n).map(|i| (sym(&format!("S{i}")), 1 << i)).collect();
        let tree = generate_huffman_tree(&pairs).expect("nonempty pairs list");
        let most = depth_of(&tree, &sym(&format!("S{}", n - 1)), 0).expect("present");
        let least = depth_of(&tree, &sym("S0"), 0).expect("present");
        (most, least)
    }

    /// Exercise 2.71: the skewed tree of a geometric frequency
    /// alphabet
    ///
    /// Merging by increasing weight always combines the two smallest
    /// leaves first, and each merged node's weight (`1 + 2 + ... +
    /// 2^(k-1) = 2^k - 1`) stays just under the next leaf's weight
    /// (`2^k`), so every merge pulls in exactly one more leaf: the
    /// tree degenerates into a single chain. The most frequent symbol
    /// (weight `2^(n-1)`) sits at depth 1; the two least frequent
    /// symbols (both weight 1) sit at the maximum depth, `n - 1`. For
    /// `n = 5`: depths 1 and 4. For `n = 10`: depths 1 and 9.
    ///
    /// Returns `(most_5, least_5, most_10, least_10)`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's two-size statement"
    )]
    pub fn ex_2_71() -> (u32, u32, u32, u32) {
        let (most_5, least_5) = depths(5);
        let (most_10, least_10) = depths(10);
        (most_5, least_5, most_10, least_10)
    }
}

#[test]
fn ex_2_71() {
    assert_eq!(ex_2_71::ex_2_71(), (1, 4, 1, 9));
}
