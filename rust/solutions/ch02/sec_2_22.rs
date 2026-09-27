// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.22, one module and one test.

mod ex_2_22 {
    use ch02::sec_2_2::{List, Nest, leaf, sub};

    /// Louis Reasoner's first rewrite: the iterative process is right,
    /// but consing the square onto the front of the accumulated answer
    /// visits the elements front to back and prepends each one, so the
    /// answer comes out in reverse.
    fn square_list_iter(items: &List<i128>) -> List<i128> {
        let mut answer = List::Nil;
        for x in items {
            answer = List::cons(x * x, &answer);
        }
        answer
    }

    /// Louis's second rewrite, `(cons answer (square (car things)))`, on
    /// the tree type: the answer goes into the `car` position, so every
    /// step wraps the growing answer and one square in a new branch
    /// instead of extending a flat list. In Rust the literal translation
    /// does not even compile: `List::cons` takes its element by value,
    /// and a `List<i128>` is not an `i128`, so the type checker rejects
    /// the swapped arguments with the mismatch Scheme only reveals at
    /// print time. [`Nest`] models what Scheme builds instead.
    fn square_list_swapped(items: &List<i128>) -> Nest<i128> {
        let mut answer = sub(&[]);
        for x in items {
            answer = sub(&[answer, leaf(x * x)]);
        }
        answer
    }

    /// Exercise 2.22: the iterative square-list bug
    ///
    /// Returns the rendered answer of Louis's first iterative rewrite
    /// (which comes out reversed) and the rendered shape his
    /// `cons`-swapped second rewrite accumulates instead of a flat list.
    pub fn ex_2_22() -> (String, String) {
        let items = List::from_iter([1, 2, 3, 4]);
        let reversed = square_list_iter(&items);
        let swapped = square_list_swapped(&items);
        (reversed.to_string(), swapped.to_string())
    }
}

#[test]
fn ex_2_22() {
    assert_eq!(
        ex_2_22::ex_2_22(),
        ("(16 9 4 1)".to_string(), "((((() 1) 4) 9) 16)".to_string())
    );
}
