// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.70, one module and one test.

mod ex_2_70 {
    use ch02::sec_2_3::{HuffmanTree, adjoin_huffman_set, encode, make_leaf_set};
    use sicp_runtime::Symbol;

    /// Exercise 2.69's `successive-merge`, reproduced locally (a
    /// separate test binary cannot import another exercise's solution
    /// module).
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

    /// Exercise 2.70: encoding a rock song's lyrics
    ///
    /// The eight-symbol alphabet's relative frequencies (`NA` 16,
    /// `YIP` 9, `SHA` 3, `A` 2, `GET` 2, `JOB` 2, `BOOM` 1, `WAH` 1)
    /// generate a Huffman tree; encoding the 36-word lyric line
    /// ("Get a job / Sha na na na na na na na na / Get a job / Sha na
    /// na na na na na na na / Wah yip yip yip yip yip yip yip yip yip
    /// / Sha boom") against it takes 84 bits. A fixed-length code for
    /// 8 symbols needs `⌈log₂ 8⌉ = 3` bits per symbol, or
    /// `3 × 36 = 108` bits for the same 36 symbols — the Huffman code
    /// saves 24 bits, about 22%.
    ///
    /// Returns the Huffman bit count and the fixed-length bit count,
    /// in that order.
    pub fn ex_2_70() -> (usize, usize) {
        let pairs = [
            (sym("A"), 2),
            (sym("NA"), 16),
            (sym("BOOM"), 1),
            (sym("SHA"), 3),
            (sym("GET"), 2),
            (sym("YIP"), 9),
            (sym("JOB"), 2),
            (sym("WAH"), 1),
        ];
        let tree = generate_huffman_tree(&pairs).expect("nonempty pairs list");

        let lyrics = [
            "GET", "A", "JOB", "SHA", "NA", "NA", "NA", "NA", "NA", "NA", "NA", "NA", "GET", "A",
            "JOB", "SHA", "NA", "NA", "NA", "NA", "NA", "NA", "NA", "NA", "WAH", "YIP", "YIP",
            "YIP", "YIP", "YIP", "YIP", "YIP", "YIP", "YIP", "SHA", "BOOM",
        ];
        let message: Vec<Symbol> = lyrics.iter().map(|w| sym(w)).collect();
        let bits = encode(&message, &tree).expect("every symbol is in the alphabet");

        let fixed_length_bits = 3 * message.len();
        (bits.len(), fixed_length_bits)
    }
}

#[test]
fn ex_2_70() {
    assert_eq!(ex_2_70::ex_2_70(), (84, 108));
}
