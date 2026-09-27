// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.67, one module and one test.

mod ex_2_67 {
    use ch02::sec_2_3::{HuffmanTree, bits_from_str, decode};
    use sicp_runtime::Symbol;

    fn sym(s: &str) -> Symbol {
        Symbol::from(s)
    }

    /// The book's `sample-tree`.
    fn sample_tree() -> HuffmanTree {
        HuffmanTree::make_code_tree(
            HuffmanTree::make_leaf(sym("A"), 4),
            HuffmanTree::make_code_tree(
                HuffmanTree::make_leaf(sym("B"), 2),
                HuffmanTree::make_code_tree(
                    HuffmanTree::make_leaf(sym("D"), 1),
                    HuffmanTree::make_leaf(sym("C"), 1),
                ),
            ),
        )
    }

    /// The book's `sample-message`, `(0 1 1 0 0 1 0 1 0 1 1 1)`.
    const SAMPLE_MESSAGE: &str = "011001010111";

    /// Exercise 2.67: decoding the sample message
    ///
    /// Returns the symbols `decode` produces for `sample-message`
    /// against `sample-tree`.
    pub fn ex_2_67() -> Vec<String> {
        let bits = bits_from_str(SAMPLE_MESSAGE).expect("well-formed bit string");
        decode(&bits, &sample_tree())
            .expect("sample-message decodes against sample-tree")
            .iter()
            .map(ToString::to_string)
            .collect()
    }
}

#[test]
fn ex_2_67() {
    assert_eq!(
        ex_2_67::ex_2_67(),
        vec![
            "A".to_string(),
            "D".to_string(),
            "A".to_string(),
            "B".to_string(),
            "B".to_string(),
            "C".to_string(),
        ]
    );
}
