// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.72, one module and one test.

mod ex_2_72 {
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

    /// `encode-symbol`, instrumented to count its dominant cost under the
    /// worst-case convention: at every node, checking which branch holds
    /// `target` is charged that branch's full symbol-list length, as if the
    /// scan ran to the end.  (The section's real `encode_symbol` early-exits
    /// on a hit; the worst-case convention reproduces the canonical SICP
    /// answer.)
    fn encode_symbol_steps(target: &Symbol, tree: &HuffmanTree, steps: &mut u64) {
        let Some(left) = tree.left_branch() else {
            return; // A leaf: encode_symbol takes no branch-membership step here.
        };
        let right = tree.right_branch().expect("a node has two branches");
        *steps += (left.symbols().len() + right.symbols().len()) as u64;
        if left.symbols().iter().any(|s| s == target) {
            encode_symbol_steps(target, left, steps);
        } else {
            encode_symbol_steps(target, right, steps);
        }
    }

    /// Counts `encode_symbol`'s steps for the most frequent symbol
    /// (`S(n-1)`) and the least frequent (`S0`) on the geometric
    /// alphabet of size `n`.
    fn steps_at(n: u32) -> (u64, u64) {
        let pairs: Vec<(Symbol, u32)> = (0..n).map(|i| (sym(&format!("S{i}")), 1 << i)).collect();
        let tree = generate_huffman_tree(&pairs).expect("nonempty pairs list");
        let mut most = 0;
        encode_symbol_steps(&sym(&format!("S{}", n - 1)), &tree, &mut most);
        let mut least = 0;
        encode_symbol_steps(&sym("S0"), &tree, &mut least);
        (most, least)
    }

    /// Exercise 2.72: order of growth of `encode-symbol`
    ///
    /// In general, `encode-symbol`'s dominant cost is the `memq`-style
    /// membership search at each node it visits, over a symbol list
    /// whose size is the size of the subtree rooted there. On
    /// exercise 2.71's skewed alphabet: the most frequent symbol sits
    /// at depth 1, and the one membership check there scans the whole
    /// `n`-symbol alphabet, so encoding it is `Θ(n)`. The least
    /// frequent symbol sits at depth `n - 1`, and the membership
    /// check at depth `k` scans a subtree of size roughly `n - k`, so
    /// the total is `1 + 2 + ... + (n - 1)`, `Θ(n²)`. Measured
    /// directly: at `n = 10`, 10 steps for the most frequent symbol
    /// versus 54 for the least; at `n = 20`, 20 versus 209 — the
    /// least-frequent count roughly quadruples when `n` doubles,
    /// confirming the quadratic growth, while the most-frequent count
    /// exactly doubles, confirming the linear growth.
    ///
    /// Returns `(most_10, least_10, most_20, least_20)`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's two-size statement"
    )]
    pub fn ex_2_72() -> (u64, u64, u64, u64) {
        let (most_10, least_10) = steps_at(10);
        let (most_20, least_20) = steps_at(20);
        (most_10, least_10, most_20, least_20)
    }
}

#[test]
fn ex_2_72() {
    assert_eq!(ex_2_72::ex_2_72(), (10, 54, 20, 209));
}
