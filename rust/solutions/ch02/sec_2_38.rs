// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.38, one module and one test.

mod ex_2_38 {
    use ch02::sec_2_2::{Nest, accumulate, leaf, sub};

    /// The book's `fold-left`, which combines elements working in the
    /// opposite direction from `accumulate` (its other name,
    /// `fold-right`): the accumulator so far is combined with each new
    /// element, left to right, in one loop.
    fn fold_left<T, A>(op: impl Fn(A, &T) -> A, initial: A, sequence: &[T]) -> A {
        let mut acc = initial;
        for x in sequence {
            acc = op(acc, x);
        }
        acc
    }

    /// Exercise 2.38: fold-left versus fold-right
    ///
    /// Returns the `fold_right` and `fold_left` divisions of `(1 2 3)`
    /// into 1, and the nested-list shapes each folding direction builds
    /// over `(1 2 3)`, in that order.
    ///
    /// Division is not associative, so the directions disagree:
    /// `3/2` against `1/6`. Nesting with the list operation agrees
    /// with itself on each side only in shape, not in orientation: the
    /// right fold hangs new elements off the right end, the left fold
    /// off the left end. The property that makes the two agree on any
    /// sequence: `op` must be associative and commutative, so the
    /// grouping and the end the initial value sits on both stop
    /// mattering.
    pub fn ex_2_38() -> (f64, f64, String, String) {
        let seq = [1.0, 2.0, 3.0];
        let right_division = accumulate(|x, acc: f64| x / acc, 1.0, &seq);
        let left_division = fold_left(|acc, x| acc / x, 1.0, &seq);

        let seq_ints = [1, 2, 3];
        let right_nesting: Nest<i128> =
            accumulate(|x, acc| sub(&[leaf(*x), acc]), sub(&[]), &seq_ints);
        let left_nesting: Nest<i128> =
            fold_left(|acc, x| sub(&[acc, leaf(*x)]), sub(&[]), &seq_ints);

        (
            right_division,
            left_division,
            right_nesting.to_string(),
            left_nesting.to_string(),
        )
    }
}

#[test]
fn ex_2_38() {
    assert_eq!(
        ex_2_38::ex_2_38(),
        (
            1.5,
            1.0 / 6.0,
            "(1 (2 (3 ())))".to_string(),
            "(((() 1) 2) 3)".to_string()
        )
    );
}
