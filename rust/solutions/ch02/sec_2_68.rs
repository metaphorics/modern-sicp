// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.68, one module and one test.

mod ex_2_68 {
    use sicp_runtime::{SchemeError, Symbol};

    /// A Huffman tree, local to this exercise: the section library's
    /// `HuffmanTree` already ships a working `encode_symbol` (needed
    /// so the library's own `encode` is a runnable listing), but this
    /// exercise's contribution is writing that procedure from
    /// scratch, per the section's running convention that a
    /// scaffold's real work stays local.
    enum HuffmanTree {
        Leaf(Symbol),
        Node(Box<HuffmanTree>, Box<HuffmanTree>, Vec<Symbol>),
    }

    fn make_leaf(symbol: Symbol) -> HuffmanTree {
        HuffmanTree::Leaf(symbol)
    }

    fn make_code_tree(left: HuffmanTree, right: HuffmanTree) -> HuffmanTree {
        let mut symbols = tree_symbols(&left);
        symbols.extend(tree_symbols(&right));
        HuffmanTree::Node(Box::new(left), Box::new(right), symbols)
    }

    fn tree_symbols(tree: &HuffmanTree) -> Vec<Symbol> {
        match tree {
            HuffmanTree::Leaf(s) => vec![s.clone()],
            HuffmanTree::Node(_, _, syms) => syms.clone(),
        }
    }

    /// `encode-symbol`: walks toward the leaf holding `symbol`,
    /// recording `false` for every left turn and `true` for every
    /// right turn, and signals an error (rather than the book's
    /// unspecified behavior) if `symbol` is not in the tree at all.
    fn encode_symbol(symbol: &Symbol, tree: &HuffmanTree) -> Result<Vec<bool>, SchemeError> {
        match tree {
            HuffmanTree::Leaf(s) if s == symbol => Ok(Vec::new()),
            HuffmanTree::Leaf(_) => Err(SchemeError::TypeMismatch(format!(
                "symbol not in tree: {symbol}"
            ))),
            HuffmanTree::Node(left, right, ..) => {
                if tree_symbols(left).iter().any(|s| s == symbol) {
                    let mut bits = vec![false];
                    bits.extend(encode_symbol(symbol, left)?);
                    Ok(bits)
                } else if tree_symbols(right).iter().any(|s| s == symbol) {
                    let mut bits = vec![true];
                    bits.extend(encode_symbol(symbol, right)?);
                    Ok(bits)
                } else {
                    Err(SchemeError::TypeMismatch(format!(
                        "symbol not in tree: {symbol}"
                    )))
                }
            }
        }
    }

    /// The book's `encode`.
    fn encode(message: &[Symbol], tree: &HuffmanTree) -> Result<Vec<bool>, SchemeError> {
        let mut bits = Vec::new();
        for symbol in message {
            bits.extend(encode_symbol(symbol, tree)?);
        }
        Ok(bits)
    }

    fn bits_to_string(bits: &[bool]) -> String {
        bits.iter().map(|&b| if b { '1' } else { '0' }).collect()
    }

    fn sym(s: &str) -> Symbol {
        Symbol::from(s)
    }

    fn sample_tree() -> HuffmanTree {
        make_code_tree(
            make_leaf(sym("A")),
            make_code_tree(
                make_leaf(sym("B")),
                make_code_tree(make_leaf(sym("D")), make_leaf(sym("C"))),
            ),
        )
    }

    /// Exercise 2.68: `encode-symbol`
    ///
    /// Returns the printed bits of encoding `[A, D, A, B, B, C]`
    /// (exercise 2.67's decoded message) back against `sample_tree`,
    /// which reproduces the original `sample_message`
    /// `011001010111`; and whether encoding a symbol absent from the
    /// tree (`Q`) is an error, in that order.
    pub fn ex_2_68() -> (String, bool) {
        let tree = sample_tree();
        let message: Vec<Symbol> = ["A", "D", "A", "B", "B", "C"]
            .into_iter()
            .map(sym)
            .collect();
        let bits = encode(&message, &tree).expect("every symbol is in sample_tree");
        let absent_is_error = encode_symbol(&sym("Q"), &tree).is_err();
        (bits_to_string(&bits), absent_is_error)
    }
}

#[test]
fn ex_2_68() {
    assert_eq!(ex_2_68::ex_2_68(), ("011001010111".to_string(), true));
}
