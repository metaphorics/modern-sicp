// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.69, one module and one test.

mod ex_2_69 {
    use ch02::sec_2_3::{HuffmanTree, adjoin_huffman_set, decode, encode, make_leaf_set};
    use sicp_runtime::Symbol;

    /// Repeatedly merges the two trees with the smallest weight,
    /// re-inserting the merged tree into the (still weight-ordered)
    /// set, until one tree remains: the book's `successive-merge`.
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

    /// Builds a Huffman encoding tree from symbol-frequency pairs: the
    /// book's `generate-huffman-tree`.
    fn generate_huffman_tree(pairs: &[(Symbol, u32)]) -> Option<HuffmanTree> {
        successive_merge(make_leaf_set(pairs))
    }

    fn sym(s: &str) -> Symbol {
        Symbol::from(s)
    }

    /// Exercise 2.69: `successive-merge` and `generate-huffman-tree`
    ///
    /// Generates a Huffman tree from the pairs `[(A, 4), (B, 2),
    /// (C, 1), (D, 1)]` (the book's `sample-tree` frequencies),
    /// encodes `[A, D, A, B, B, C]` against it, and returns the
    /// number of bits used and whether decoding those bits reproduces
    /// the original message, in that order. The generated tree's
    /// leaf depths, and so its total code length, match `sample-tree`
    /// exactly; only the left/right choice for the tied `C`/`D` merge
    /// can differ, which the book's own footnote calls arbitrary, so
    /// this checks the generated tree's self-consistency instead of
    /// an exact bit-for-bit match with the hand-built `sample-tree`.
    pub fn ex_2_69() -> (usize, bool) {
        let pairs = [(sym("A"), 4), (sym("B"), 2), (sym("C"), 1), (sym("D"), 1)];
        let tree = generate_huffman_tree(&pairs).expect("nonempty pairs list");
        let message: Vec<Symbol> = ["A", "D", "A", "B", "B", "C"]
            .into_iter()
            .map(sym)
            .collect();
        let bits = encode(&message, &tree).expect("every symbol is in the generated tree");
        let decoded = decode(&bits, &tree).expect("well-formed encoding");
        (bits.len(), decoded == message)
    }
}

#[test]
fn ex_2_69() {
    assert_eq!(ex_2_69::ex_2_69(), (12, true));
}
