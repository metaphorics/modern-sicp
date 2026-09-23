// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.26, one module and one test.

mod ex_2_26 {
    use ch02::sec_2_2::{List, Nest, append, leaf};
    use std::rc::Rc;

    /// A flat list of integers seen as a tree: the one-element-deep
    /// Nest whose branch holds the list's elements as leaves.
    fn as_tree(l: &List<i128>) -> Nest<i128> {
        Nest::Sub(Rc::new(l.iter().map(|x| leaf(*x)).collect()))
    }

    /// Exercise 2.26: append, cons, and list on two lists
    ///
    /// With `x = (1 2 3)` and `y = (4 5 6)`: `append(x, y)` copies
    /// `x`'s spine and ends it in `y`, so the result is a flat
    /// `List<i128>`, `(1 2 3 4 5 6)`. `list(x, y)` builds a two-element
    /// list of the two lists, a `List<List<i128>>` that prints
    /// `((1 2 3) (4 5 6))`. `cons(x, y)` is the instructive case: Scheme
    /// pairs the list `x` with the list `y`, mixing element types in one
    /// structure, which no single homogeneous Rust `List<T>` can hold;
    /// the edition's tree type is its home, and it prints exactly the
    /// book's `((1 2 3) 4 5 6)`.
    pub fn ex_2_26() -> (String, String, String) {
        let x: List<i128> = List::from_iter([1, 2, 3]);
        let y: List<i128> = List::from_iter([4, 5, 6]);

        let appended = append(&x, &y);
        let listed: List<List<i128>> = List::from_iter([x.clone(), y.clone()]);
        let y_leaves: List<Nest<i128>> = y.iter().map(|v| leaf(*v)).collect();
        let consed = Nest::Sub(Rc::new(List::cons(as_tree(&x), &y_leaves)));

        (appended.to_string(), consed.to_string(), listed.to_string())
    }
}

#[test]
fn ex_2_26() {
    assert_eq!(
        ex_2_26::ex_2_26(),
        (
            "(1 2 3 4 5 6)".to_string(),
            "((1 2 3) 4 5 6)".to_string(),
            "((1 2 3) (4 5 6))".to_string()
        )
    );
}
