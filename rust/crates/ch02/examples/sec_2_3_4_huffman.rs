// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.3.4

//! Section 2.3.4: Huffman encoding trees, `make-leaf-set`, `decode`, and
//! `encode`.

use ch02::sec_2_3::{HuffmanTree, bits_from_str, bits_to_string, decode, encode, make_leaf_set};
use sicp_runtime::Symbol;

fn sym(s: &str) -> Symbol {
    Symbol::from(s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `make-leaf-set` turns symbol-frequency pairs into an initial set
    // of leaves ordered by increasing weight, ready to be merged
    // according to the Huffman algorithm (exercise 2.69 writes the
    // merge itself).
    let pairs = [(sym("A"), 4), (sym("B"), 2), (sym("C"), 1), (sym("D"), 1)];
    let leaves = make_leaf_set(&pairs);
    let weights: Vec<u32> = leaves.iter().map(HuffmanTree::weight).collect();
    println!("{weights:?}");
    // => [1, 1, 2, 4]
    assert_eq!(weights, vec![1, 1, 2, 4]);

    // The book's `sample-tree`, built directly rather than merged.
    let sample_tree = HuffmanTree::make_code_tree(
        HuffmanTree::make_leaf(sym("A"), 4),
        HuffmanTree::make_code_tree(
            HuffmanTree::make_leaf(sym("B"), 2),
            HuffmanTree::make_code_tree(
                HuffmanTree::make_leaf(sym("D"), 1),
                HuffmanTree::make_leaf(sym("C"), 1),
            ),
        ),
    );

    let bits = bits_from_str("011001010111")?;
    let decoded = decode(&bits, &sample_tree)?;
    let names: Vec<String> = decoded.iter().map(ToString::to_string).collect();
    println!("{}", names.join(" "));
    // => A D A B B C
    assert_eq!(names, vec!["A", "D", "A", "B", "B", "C"]);

    // `encode` inverts `decode`: encoding the decoded message reproduces
    // the original bit string exactly.
    let re_encoded = encode(&decoded, &sample_tree)?;
    println!("{}", bits_to_string(&re_encoded));
    // => 011001010111
    assert_eq!(bits_to_string(&re_encoded), "011001010111");

    Ok(())
}
