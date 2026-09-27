// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.3

//! Section 2.3: Symbolic data.
//!
//! Quotation disappears into constructors: a symbol is a Rust string
//! literal converted to [`Symbol`], and a quoted list is built with
//! [`crate::sec_2_2::leaf`] and [`crate::sec_2_2::sub`] rather than read
//! by an interpreter. `eq?` on symbols becomes `Symbol` (`Rc<str>`)
//! comparison. Symbolic differentiation (2.3.2) represents expressions
//! as the closed enum [`Expr`]; `deriv` is an exhaustive `match`, so the
//! book's `(error "unknown expression type: DERIV" exp)` case is
//! structurally unreachable. Sets (2.3.3) are `Vec<i128>` for the
//! unordered and ordered representations and a binary-search-tree enum
//! for the third; the book restricts sets to numbers once it reaches the
//! ordered representation, and this edition uses `i128` throughout for
//! one consistent element type. Huffman trees (2.3.4) are an enum
//! carrying aggregate weight and symbol set at every node, and a bit is
//! a `bool` rather than an unconstrained value, so `choose-branch`'s
//! `(error "bad bit...")` case is likewise structurally unreachable.

use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use sicp_runtime::{SchemeError, Symbol};

use crate::sec_2_2::List;

// ---------------------------------------------------------------------
// 2.3.1 Quotation
// ---------------------------------------------------------------------

/// Finds `item` in `xs` by `eq?`-style comparison: the book's `memq`.
/// For `T = Symbol` (`Rc<str>`), `==` is a content compare, exactly the
/// book's `eq?` on symbols; the same procedure works over any
/// comparable element, including the nested-list case of exercise
/// 2.53. Returns the sublist beginning at the first match, or `None`
/// when `item` does not occur.
#[must_use]
pub fn memq<T: PartialEq + Clone>(item: &T, xs: &List<T>) -> Option<List<T>> {
    let mut cursor = xs;
    loop {
        match cursor {
            List::Nil => return None,
            List::Cons(x, rest) => {
                if x == item {
                    return Some(cursor.clone());
                }
                cursor = rest;
            }
        }
    }
}

// ---------------------------------------------------------------------
// 2.3.2 Example: Symbolic differentiation
// ---------------------------------------------------------------------

/// An algebraic expression: the book's parenthesized prefix notation,
/// closed as an enum instead of open list structure. The book's seven
/// predicates and selectors (`variable?`, `same-variable?`, `sum?`,
/// `addend`, `augend`, `product?`, `multiplier`, `multiplicand`)
/// collapse into this type's variants and fields; a `match` on `Expr`
/// is the predicate dispatch, and the compiler's exhaustiveness check
/// proves every case of `deriv` is handled, which is the section's
/// promised abstraction-barrier property made structural.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A number.
    Num(i128),
    /// A variable, identified by its symbol.
    Var(Symbol),
    /// A sum of two expressions.
    Sum(Box<Expr>, Box<Expr>),
    /// A product of two expressions.
    Product(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    /// Prints the book's parenthesized prefix notation: `(+ a b)` for a
    /// sum, `(* a b)` for a product, so a derivative's printed form
    /// matches the book's interaction transcripts exactly.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Var(x) => write!(f, "{x}"),
            Expr::Sum(a, b) => write!(f, "(+ {a} {b})"),
            Expr::Product(a, b) => write!(f, "(* {a} {b})"),
        }
    }
}

/// Checks whether an expression is equal to a given number: the book's
/// `=number?`.
#[must_use]
pub fn is_number(exp: &Expr, num: i128) -> bool {
    matches!(exp, Expr::Num(x) if *x == num)
}

/// Checks whether `exp` is the variable `var`: the book's
/// `(same-variable? exp var)`, called from inside `deriv` where `exp`
/// is the expression under the `Var` case and `var` is the
/// differentiation variable.
#[must_use]
pub fn is_same_variable(exp: &Expr, var: &Symbol) -> bool {
    matches!(exp, Expr::Var(x) if x == var)
}

/// Builds a sum without simplifying it: the book's first, naive
/// `make-sum`, always `(+ a1 a2)`.
#[must_use]
pub fn make_sum_unsimplified(a1: Expr, a2: Expr) -> Expr {
    Expr::Sum(Box::new(a1), Box::new(a2))
}

/// Builds a product without simplifying it: the book's first, naive
/// `make-product`, always `(* m1 m2)`.
#[must_use]
pub fn make_product_unsimplified(m1: Expr, m2: Expr) -> Expr {
    Expr::Product(Box::new(m1), Box::new(m2))
}

/// Builds a sum, folding the rules that `0 + a2 = a2`, `a1 + 0 = a1`,
/// and the sum of two numbers is their numeric sum: the book's refined
/// `make-sum`.
///
/// # Errors
/// [`SchemeError::Overflow`] when both summands are numbers whose sum
/// leaves the `i128` range.
pub fn make_sum(a1: Expr, a2: Expr) -> Result<Expr, SchemeError> {
    if is_number(&a1, 0) {
        return Ok(a2);
    }
    if is_number(&a2, 0) {
        return Ok(a1);
    }
    if let (Expr::Num(x), Expr::Num(y)) = (&a1, &a2) {
        return Ok(Expr::Num(x.checked_add(*y).ok_or(SchemeError::Overflow)?));
    }
    Ok(Expr::Sum(Box::new(a1), Box::new(a2)))
}

/// Builds a product, folding the rules that `0 * anything = 0`,
/// `1 * m2 = m2`, `m1 * 1 = m1`, and the product of two numbers is
/// their numeric product: the book's refined `make-product`.
///
/// # Errors
/// [`SchemeError::Overflow`] when both factors are numbers whose
/// product leaves the `i128` range.
pub fn make_product(m1: Expr, m2: Expr) -> Result<Expr, SchemeError> {
    if is_number(&m1, 0) || is_number(&m2, 0) {
        return Ok(Expr::Num(0));
    }
    if is_number(&m1, 1) {
        return Ok(m2);
    }
    if is_number(&m2, 1) {
        return Ok(m1);
    }
    if let (Expr::Num(x), Expr::Num(y)) = (&m1, &m2) {
        return Ok(Expr::Num(x.checked_mul(*y).ok_or(SchemeError::Overflow)?));
    }
    Ok(Expr::Product(Box::new(m1), Box::new(m2)))
}

/// Differentiates `exp` with respect to `var`, combining sub-derivatives
/// through the two constructors passed in. The book says "we won't
/// change `deriv` at all. Instead, we will change `make-sum`"; in Rust
/// that promise is literal: `deriv` never changes, and passing the
/// naive or the simplifying pair of constructors is the entire
/// difference between [`deriv_unsimplified`] and [`deriv`].
///
/// # Errors
/// Propagates whatever `make_sum` or `make_product` returns.
pub fn deriv_via(
    exp: &Expr,
    var: &Symbol,
    make_sum: &impl Fn(Expr, Expr) -> Result<Expr, SchemeError>,
    make_product: &impl Fn(Expr, Expr) -> Result<Expr, SchemeError>,
) -> Result<Expr, SchemeError> {
    match exp {
        Expr::Num(_) => Ok(Expr::Num(0)),
        Expr::Var(x) => Ok(Expr::Num(i128::from(x == var))),
        Expr::Sum(a1, a2) => make_sum(
            deriv_via(a1, var, make_sum, make_product)?,
            deriv_via(a2, var, make_sum, make_product)?,
        ),
        Expr::Product(m1, m2) => {
            let left = make_product((**m1).clone(), deriv_via(m2, var, make_sum, make_product)?)?;
            let right = make_product(deriv_via(m1, var, make_sum, make_product)?, (**m2).clone())?;
            make_sum(left, right)
        }
    }
}

/// Differentiates `exp` with respect to `var`, without simplifying: the
/// book's first `deriv`, run against the naive constructors. Never
/// actually errors, since the naive constructors never fold numbers,
/// but shares [`deriv_via`]'s signature.
///
/// # Errors
/// Never, in practice; see above.
pub fn deriv_unsimplified(exp: &Expr, var: &Symbol) -> Result<Expr, SchemeError> {
    deriv_via(
        exp,
        var,
        &|a, b| Ok(make_sum_unsimplified(a, b)),
        &|a, b| Ok(make_product_unsimplified(a, b)),
    )
}

/// Differentiates `exp` with respect to `var`, simplifying as it goes:
/// the book's `deriv`, run against the refined constructors.
///
/// # Errors
/// [`SchemeError::Overflow`] when a numeric fold leaves the `i128`
/// range.
pub fn deriv(exp: &Expr, var: &Symbol) -> Result<Expr, SchemeError> {
    deriv_via(exp, var, &make_sum, &make_product)
}

// ---------------------------------------------------------------------
// 2.3.3 Example: Representing sets
// ---------------------------------------------------------------------
//
// The book restricts sets to numbers once it reaches the ordered-list
// representation, so it can compare elements with `<`/`>`; this edition
// uses `i128` for all three representations, unordered included, for
// one consistent element type.

/// Sets as unordered lists: is `x` a member of `set`? The book's
/// `element-of-set?`, which scans the whole set in the worst case.
#[must_use]
pub fn element_of_set(x: i128, set: &[i128]) -> bool {
    set.contains(&x)
}

/// Sets as unordered lists: adjoins `x` to `set`, or returns `set`
/// unchanged if `x` is already present.
#[must_use]
pub fn adjoin_set(x: i128, set: &[i128]) -> Vec<i128> {
    if element_of_set(x, set) {
        set.to_vec()
    } else {
        let mut out = vec![x];
        out.extend_from_slice(set);
        out
    }
}

/// Sets as unordered lists: the elements common to `set1` and `set2`,
/// in `set1`'s order. `Θ(n²)` for sets of size `n`, since every element
/// of `set1` scans all of `set2`.
#[must_use]
pub fn intersection_set(set1: &[i128], set2: &[i128]) -> Vec<i128> {
    set1.iter()
        .copied()
        .filter(|&x| element_of_set(x, set2))
        .collect()
}

/// Sets as ordered lists (elements increasing): is `x` a member of
/// `set`? Stops scanning once a larger element is reached, so the
/// average case is about half the unordered representation's, though
/// the worst case (the largest element, or absence) is unchanged.
#[must_use]
pub fn element_of_set_ordered(x: i128, set: &[i128]) -> bool {
    match set.first() {
        None => false,
        Some(&y) => match x.cmp(&y) {
            Ordering::Equal => true,
            Ordering::Less => false,
            Ordering::Greater => element_of_set_ordered(x, &set[1..]),
        },
    }
}

/// Sets as ordered lists: the elements common to `set1` and `set2`.
/// Walking both lists in step, taking the smaller head (or the shared
/// head once, advancing both) reduces the problem to computing the
/// intersection of smaller sets at each step, removing an element from
/// one or both lists, so this is `Θ(n)` rather than the unordered
/// representation's `Θ(n²)`. The book writes this recursively, which
/// is `Θ(n)` in Scheme because `cons` shares structure; this edition
/// writes the same walk as a loop so the `Vec` result is built once,
/// keeping the `Θ(n)` cost real rather than an artifact of a shared
/// list the host type does not have.
#[must_use]
pub fn intersection_set_ordered(set1: &[i128], set2: &[i128]) -> Vec<i128> {
    let mut out = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < set1.len() && j < set2.len() {
        match set1[i].cmp(&set2[j]) {
            Ordering::Equal => {
                out.push(set1[i]);
                i += 1;
                j += 1;
            }
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
        }
    }
    out
}

/// Sets as binary trees: a node holds one element (the "entry"), a left
/// subtree of smaller elements, and a right subtree of larger ones, or
/// `Empty`. The book represents this shape with three-item lists
/// (`entry`, `left-branch`, `right-branch`); this edition represents it
/// directly as a recursive enum, which is the abstraction the book's
/// list encoding was standing in for. Branches are `Rc`, not `Box`,
/// for the same reason [`List`] uses `Rc`: adjoining an element
/// rebuilds the spine from the insertion point to the root and shares
/// every untouched sibling subtree by pointer, so the `Θ(log n)`
/// claims below are genuine rather than hidden behind an `O(n)` deep
/// clone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    /// No elements.
    Empty,
    /// One element, with its left and right subtrees.
    Node(i128, Rc<Tree>, Rc<Tree>),
}

impl Tree {
    /// Builds a tree node: the book's `make-tree`.
    #[must_use]
    pub fn make_tree(entry: i128, left: Tree, right: Tree) -> Tree {
        Tree::Node(entry, Rc::new(left), Rc::new(right))
    }

    /// The entry at the root, or `None` on `Empty`: the book's `entry`.
    #[must_use]
    pub fn entry(&self) -> Option<i128> {
        match self {
            Tree::Node(x, ..) => Some(*x),
            Tree::Empty => None,
        }
    }

    /// The left subtree, or `None` on `Empty`: the book's `left-branch`.
    #[must_use]
    pub fn left_branch(&self) -> Option<&Tree> {
        match self {
            Tree::Node(_, l, _) => Some(l),
            Tree::Empty => None,
        }
    }

    /// The right subtree, or `None` on `Empty`: the book's
    /// `right-branch`.
    #[must_use]
    pub fn right_branch(&self) -> Option<&Tree> {
        match self {
            Tree::Node(_, _, r) => Some(r),
            Tree::Empty => None,
        }
    }
}

impl fmt::Display for Tree {
    /// Prints the book's `(entry left right)` list form, so an empty
    /// subtree prints `()`, matching the figures.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Empty => write!(f, "()"),
            Tree::Node(x, l, r) => write!(f, "({x} {l} {r})"),
        }
    }
}

/// Sets as binary trees: is `x` a member of `tree`? Halving the
/// problem at each step (assuming the tree stays balanced) makes this
/// `Θ(log n)`, a significant speedup over both list representations.
#[must_use]
pub fn element_of_set_tree(x: i128, tree: &Tree) -> bool {
    match tree {
        Tree::Empty => false,
        Tree::Node(entry, left, right) => match x.cmp(entry) {
            Ordering::Equal => true,
            Ordering::Less => element_of_set_tree(x, left),
            Ordering::Greater => element_of_set_tree(x, right),
        },
    }
}

/// Sets as binary trees: adjoins `x` to `tree`, rebuilding the spine
/// from the insertion point back to the root. Also `Θ(log n)` for a
/// balanced tree, though repeated adjoining does not guarantee the
/// tree stays balanced (exercises 2.63-2.65 address this).
#[must_use]
pub fn adjoin_set_tree(x: i128, tree: &Tree) -> Tree {
    match tree {
        Tree::Empty => Tree::make_tree(x, Tree::Empty, Tree::Empty),
        Tree::Node(entry, left, right) => match x.cmp(entry) {
            Ordering::Equal => tree.clone(),
            Ordering::Less => Tree::make_tree(*entry, adjoin_set_tree(x, left), (**right).clone()),
            Ordering::Greater => {
                Tree::make_tree(*entry, (**left).clone(), adjoin_set_tree(x, right))
            }
        },
    }
}

// ---------------------------------------------------------------------
// 2.3.4 Example: Huffman encoding trees
// ---------------------------------------------------------------------

/// A Huffman encoding tree: a leaf holds one symbol and its weight; a
/// node holds its two branches plus the aggregate symbol set and
/// weight of everything below it, computed once at construction rather
/// than walked on every query. The book represents both cases as
/// tagged lists (`leaf` versus untagged); this edition closes the two
/// shapes as enum variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HuffmanTree {
    /// A leaf: one symbol and its weight.
    Leaf(Symbol, u32),
    /// A node: its two branches, the symbols reachable below it, and
    /// their total weight.
    Node(Box<HuffmanTree>, Box<HuffmanTree>, Vec<Symbol>, u32),
}

impl HuffmanTree {
    /// Builds a leaf: the book's `make-leaf`.
    #[must_use]
    pub fn make_leaf(symbol: Symbol, weight: u32) -> HuffmanTree {
        HuffmanTree::Leaf(symbol, weight)
    }

    /// Whether this tree is a leaf: the book's `leaf?`.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        matches!(self, HuffmanTree::Leaf(..))
    }

    /// The symbol at a leaf, or `None` on a node: the book's
    /// `symbol-leaf`.
    #[must_use]
    pub fn symbol_leaf(&self) -> Option<&Symbol> {
        match self {
            HuffmanTree::Leaf(s, _) => Some(s),
            HuffmanTree::Node(..) => None,
        }
    }

    /// Merges two subtrees into a node, computing its aggregate symbol
    /// set and weight: the book's `make-code-tree`.
    #[must_use]
    pub fn make_code_tree(left: HuffmanTree, right: HuffmanTree) -> HuffmanTree {
        let mut symbols = left.symbols();
        symbols.extend(right.symbols());
        let weight = left.weight() + right.weight();
        HuffmanTree::Node(Box::new(left), Box::new(right), symbols, weight)
    }

    /// The left branch, or `None` on a leaf: the book's `left-branch`,
    /// re-defined here for the Huffman tree's own list shape.
    #[must_use]
    pub fn left_branch(&self) -> Option<&HuffmanTree> {
        match self {
            HuffmanTree::Node(l, ..) => Some(l),
            HuffmanTree::Leaf(..) => None,
        }
    }

    /// The right branch, or `None` on a leaf: the book's
    /// `right-branch`, re-defined here for the Huffman tree.
    #[must_use]
    pub fn right_branch(&self) -> Option<&HuffmanTree> {
        match self {
            HuffmanTree::Node(_, r, ..) => Some(r),
            HuffmanTree::Leaf(..) => None,
        }
    }

    /// The symbols reachable below this tree: a leaf contributes its
    /// one symbol; a node returns its precomputed set. The book's
    /// `symbols` dispatches on `leaf?` and reads `caddr` on a node;
    /// this edition stores the set instead of reconstructing it.
    #[must_use]
    pub fn symbols(&self) -> Vec<Symbol> {
        match self {
            HuffmanTree::Leaf(s, _) => vec![Rc::clone(s)],
            HuffmanTree::Node(_, _, syms, _) => syms.clone(),
        }
    }

    /// The total weight below this tree: the book's `weight`.
    #[must_use]
    pub fn weight(&self) -> u32 {
        match self {
            HuffmanTree::Leaf(_, w) | HuffmanTree::Node(_, _, _, w) => *w,
        }
    }
}

/// Moves one step down a Huffman tree by one bit: `false` (the book's
/// `0`) takes the left branch, `true` (the book's `1`) takes the
/// right. Returns `None` when `branch` is already a leaf, which
/// [`decode`] never lets happen: a `bool` has no third value, so the
/// book's `(error "bad bit..." bit)` case is structurally unreachable
/// here, unlike the leaf-with-no-more-bits case this signature still
/// has to name.
#[must_use]
pub fn choose_branch(bit: bool, branch: &HuffmanTree) -> Option<&HuffmanTree> {
    if bit {
        branch.right_branch()
    } else {
        branch.left_branch()
    }
}

/// Decodes a bit sequence against a Huffman tree: the book's `decode`.
/// Each bit moves one step down the tree from the current position;
/// reaching a leaf emits its symbol and restarts at the root. The
/// book's `decode-1` is a second recursive procedure only because
/// Scheme has no loop construct of its own; this edition expresses the
/// same restart-at-the-root behavior as a loop.
///
/// # Errors
/// [`SchemeError::Parse`] if a bit is consumed at a leaf, which means
/// `bits` and `tree` are inconsistent (more bits than the encoding
/// produced).
pub fn decode(bits: &[bool], tree: &HuffmanTree) -> Result<Vec<Symbol>, SchemeError> {
    let mut result = Vec::new();
    let mut current = tree;
    for &bit in bits {
        let next = choose_branch(bit, current)
            .ok_or_else(|| SchemeError::Parse("decode: bit consumed at a leaf".to_string()))?;
        if let HuffmanTree::Leaf(symbol, _) = next {
            result.push(symbol.clone());
            current = tree;
        } else {
            current = next;
        }
    }
    Ok(result)
}

/// Adjoins a tree to a set of trees ordered by increasing weight: the
/// book's `adjoin-set`, re-defined here for Huffman trees (unlike the
/// number sets of 2.3.3, an element here is never already present, so
/// there is no membership check).
#[must_use]
pub fn adjoin_huffman_set(x: HuffmanTree, set: &[HuffmanTree]) -> Vec<HuffmanTree> {
    match set.split_first() {
        None => vec![x],
        Some((first, rest)) if x.weight() < first.weight() => {
            let mut out = vec![x, first.clone()];
            out.extend_from_slice(rest);
            out
        }
        Some((first, rest)) => {
            let mut out = vec![first.clone()];
            out.extend(adjoin_huffman_set(x, rest));
            out
        }
    }
}

/// Builds an initial ordered set of leaves from symbol-frequency pairs,
/// ready to be merged according to the Huffman algorithm: the book's
/// `make-leaf-set`.
#[must_use]
pub fn make_leaf_set(pairs: &[(Symbol, u32)]) -> Vec<HuffmanTree> {
    let mut set = Vec::new();
    for (symbol, weight) in pairs {
        set = adjoin_huffman_set(HuffmanTree::make_leaf(Rc::clone(symbol), *weight), &set);
    }
    set
}

/// Encodes one symbol as the list of bits that reaches its leaf: the
/// book gives `encode`'s shell but leaves `encode-symbol` as exercise
/// 2.68's contribution. This edition's library ships a working version
/// so [`encode`] below is a runnable book listing; exercise 2.68's own
/// solution writes its own independent copy, per the section's running
/// convention that a scaffold's real work stays local to the exercise.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] if `symbol` does not appear in `tree`.
pub fn encode_symbol(symbol: &Symbol, tree: &HuffmanTree) -> Result<Vec<bool>, SchemeError> {
    match tree {
        HuffmanTree::Leaf(s, _) if s == symbol => Ok(Vec::new()),
        HuffmanTree::Leaf(..) => Err(SchemeError::TypeMismatch(format!(
            "symbol not in tree: {symbol}"
        ))),
        HuffmanTree::Node(left, right, ..) => {
            if left.symbols().iter().any(|s| s == symbol) {
                let mut bits = vec![false];
                bits.extend(encode_symbol(symbol, left)?);
                Ok(bits)
            } else if right.symbols().iter().any(|s| s == symbol) {
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

/// Encodes a message (a sequence of symbols) into its bit list, using
/// `tree`: the book's `encode`.
///
/// # Errors
/// Propagates [`encode_symbol`]'s error for any symbol not in `tree`.
pub fn encode(message: &[Symbol], tree: &HuffmanTree) -> Result<Vec<bool>, SchemeError> {
    let mut bits = Vec::new();
    for symbol in message {
        bits.extend(encode_symbol(symbol, tree)?);
    }
    Ok(bits)
}

/// Parses a string of `0`/`1` characters into bits, in order: the
/// compact way this edition's examples and tests write Huffman
/// messages, e.g. `"0110010101111"`.
///
/// # Errors
/// [`SchemeError::Parse`] on any character other than `0` or `1`.
pub fn bits_from_str(s: &str) -> Result<Vec<bool>, SchemeError> {
    s.chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            other => Err(SchemeError::Parse(format!("not a bit: {other}"))),
        })
        .collect()
}

/// Prints bits as a string of `0`/`1` characters, in order: the
/// inverse of [`bits_from_str`].
#[must_use]
pub fn bits_to_string(bits: &[bool]) -> String {
    bits.iter().map(|&b| if b { '1' } else { '0' }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sym(s: &str) -> Symbol {
        Symbol::from(s)
    }

    #[test]
    fn memq_finds_the_sublist_starting_at_the_match() {
        let xs: List<Symbol> = List::from_iter([sym("apple"), sym("pear"), sym("banana")]);
        let found = memq(&sym("pear"), &xs).expect("present");
        assert_eq!(found.to_string(), "(pear banana)");
        assert!(memq(&sym("plum"), &xs).is_none());
    }

    #[test]
    fn deriv_unsimplified_matches_the_books_first_examples() {
        let x = sym("x");
        let e1 = Expr::Sum(Box::new(Expr::Var(sym("x"))), Box::new(Expr::Num(3)));
        assert_eq!(deriv_unsimplified(&e1, &x).unwrap().to_string(), "(+ 1 0)");

        let e2 = Expr::Product(Box::new(Expr::Var(sym("x"))), Box::new(Expr::Var(sym("y"))));
        assert_eq!(
            deriv_unsimplified(&e2, &x).unwrap().to_string(),
            "(+ (* x 0) (* 1 y))"
        );
    }

    #[test]
    fn deriv_simplifies_to_the_books_second_examples() {
        let x = sym("x");
        let e1 = Expr::Sum(Box::new(Expr::Var(sym("x"))), Box::new(Expr::Num(3)));
        assert_eq!(deriv(&e1, &x).unwrap().to_string(), "1");

        let e2 = Expr::Product(Box::new(Expr::Var(sym("x"))), Box::new(Expr::Var(sym("y"))));
        assert_eq!(deriv(&e2, &x).unwrap().to_string(), "y");

        let e3 = Expr::Product(
            Box::new(Expr::Product(
                Box::new(Expr::Var(sym("x"))),
                Box::new(Expr::Var(sym("y"))),
            )),
            Box::new(Expr::Sum(
                Box::new(Expr::Var(sym("x"))),
                Box::new(Expr::Num(3)),
            )),
        );
        assert_eq!(
            deriv(&e3, &x).unwrap().to_string(),
            "(+ (* x y) (* y (+ x 3)))"
        );
    }

    #[test]
    fn unordered_set_operations_match_the_book() {
        assert!(element_of_set(3, &[1, 2, 3]));
        assert!(!element_of_set(4, &[1, 2, 3]));
        assert_eq!(adjoin_set(4, &[1, 2, 3]), vec![4, 1, 2, 3]);
        assert_eq!(adjoin_set(2, &[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(intersection_set(&[1, 2, 3], &[2, 3, 4]), vec![2, 3]);
    }

    #[test]
    fn ordered_set_operations_match_the_book() {
        assert!(element_of_set_ordered(3, &[1, 3, 6, 10]));
        assert!(!element_of_set_ordered(4, &[1, 3, 6, 10]));
        assert_eq!(
            intersection_set_ordered(&[1, 3, 5, 7], &[3, 5, 9]),
            vec![3, 5]
        );
    }

    #[test]
    fn tree_set_operations_match_the_book() {
        let tree = Tree::make_tree(7, Tree::make_tree(3, Tree::Empty, Tree::Empty), Tree::Empty);
        assert!(element_of_set_tree(3, &tree));
        assert!(!element_of_set_tree(9, &tree));
        let grown = adjoin_set_tree(9, &tree);
        assert!(element_of_set_tree(9, &grown));
        assert_eq!(grown.to_string(), "(7 (3 () ()) (9 () ()))");
    }

    #[test]
    fn huffman_decode_matches_the_books_sample() {
        let tree = HuffmanTree::make_code_tree(
            HuffmanTree::make_leaf(sym("A"), 4),
            HuffmanTree::make_code_tree(
                HuffmanTree::make_leaf(sym("B"), 2),
                HuffmanTree::make_code_tree(
                    HuffmanTree::make_leaf(sym("D"), 1),
                    HuffmanTree::make_leaf(sym("C"), 1),
                ),
            ),
        );
        let bits = bits_from_str("011001010111").unwrap();
        let decoded = decode(&bits, &tree).unwrap();
        let names: Vec<String> = decoded.iter().map(ToString::to_string).collect();
        assert_eq!(names, vec!["A", "D", "A", "B", "B", "C"]);
    }

    #[test]
    fn huffman_encode_round_trips_through_decode() {
        let tree = HuffmanTree::make_code_tree(
            HuffmanTree::make_leaf(sym("A"), 4),
            HuffmanTree::make_code_tree(
                HuffmanTree::make_leaf(sym("B"), 2),
                HuffmanTree::make_code_tree(
                    HuffmanTree::make_leaf(sym("D"), 1),
                    HuffmanTree::make_leaf(sym("C"), 1),
                ),
            ),
        );
        let message: Vec<Symbol> = ["A", "D", "A", "B", "B", "C"]
            .into_iter()
            .map(sym)
            .collect();
        let bits = encode(&message, &tree).unwrap();
        assert_eq!(bits_to_string(&bits), "011001010111");
        let decoded = decode(&bits, &tree).unwrap();
        assert_eq!(decoded, message);
    }
}
