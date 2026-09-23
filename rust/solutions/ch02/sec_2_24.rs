// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.24, one module and one test.
//!
//! SICP exercise 2.24 asks for the printed result, the box-and-pointer
//! structure, and the tree interpretation of a nested `list` expression.
//! The printed result is the part Scheme runs for you, so this edition's
//! replacement keeps that part runnable and carries the two drawings in
//! the comments below, in the edition's own syntax.
//!
//! ```text
//! Box and pointer for `list(1, list(2, list(3, 4)))`, outermost cell
//! first, `/` marking the end-of-list slot:
//!
//! (1 (2 (3 4)))  +---+---+     +---+---+     +---+---+     +---+---+
//!           ---> | * | *-+---->| * | *-+---->| * | *-+---->| * | / |
//!                +-|-+---+     +-|-+---+     +-|-+---+     +-|-+---+
//!                  |             |             |             |
//!                  V             V             |             V
//!                +---+         +---+---+       |           +---+
//!                | 1 |         | * | *-+---+   |           | 4 |
//!                +---+         +-|-+---+   |   |           +---+
//!                                V         |   V
//!                              +---+       | +---+---+
//!                              | 2 |       | | * | / |
//!                              +---+       | +-|-+---+
//!                                          |   V
//!                                          | +---+---+
//!                                          +-| * | *-+
//!                                            +-|-----+
//!                                              |   (the innermost
//!                                              V    two-element list)
//!                                            +---+
//!                                            | 3 |
//!                                            +---+
//!
//! Tree interpretation: the root branch holds the leaves `1` and the
//! subtree `(2 (3 4))`; that subtree holds `2` and `(3 4)`; the deepest
//! subtree holds the leaves `3` and `4`.
//!
//!            (1 (2 (3 4)))
//!              /       \
//!             1      (2 (3 4))
//!                    /      \
//!                   2     (3 4)
//!                         /    \
//!                        3      4
//! ```

mod ex_2_24 {
    use ch02::sec_2_2::{leaf, sub};

    /// Exercise 2.24: the structure of a nested list
    ///
    /// Builds `list(1, list(2, list(3, 4)))` in the edition's tree type
    /// and returns its printed form.
    pub fn ex_2_24() -> String {
        let tree = sub(&[leaf(1), sub(&[leaf(2), sub(&[leaf(3), leaf(4)])])]);
        tree.to_string()
    }
}

#[test]
fn ex_2_24() {
    assert_eq!(ex_2_24::ex_2_24(), "(1 (2 (3 4)))".to_string());
}
