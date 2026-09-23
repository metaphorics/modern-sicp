// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.39, one module and one test.

mod ex_2_39 {
    use ch02::sec_2_2::{List, accumulate, append};

    /// Exercise 2.39: `reverse` with `fold-right`: the accumulation of
    /// the rest is built first, so each element must be appended at the
    /// far (right) end of the answer.
    fn reverse_right(sequence: &[i128]) -> List<i128> {
        accumulate(
            |x, acc: List<i128>| append(&acc, &List::cons(*x, &List::Nil)),
            List::Nil,
            sequence,
        )
    }

    /// Exercise 2.39: `reverse` with `fold-left`: each element arrives
    /// in order and is consed onto the front of the answer built so
    /// far, so the first element ends up last.
    fn reverse_left(sequence: &[i128]) -> List<i128> {
        let mut acc = List::Nil;
        for x in sequence {
            acc = List::cons(*x, &acc);
        }
        acc
    }

    /// Exercise 2.39: reverse via folds
    ///
    /// Returns the rendered reverse of `(1 2 3 4)` built with
    /// `fold_right` and with `fold_left`, in that order.
    pub fn ex_2_39() -> (String, String) {
        let seq = [1, 2, 3, 4];
        let right = reverse_right(&seq);
        let left = reverse_left(&seq);
        (right.to_string(), left.to_string())
    }
}

#[test]
fn ex_2_39() {
    assert_eq!(
        ex_2_39::ex_2_39(),
        ("(4 3 2 1)".to_string(), "(4 3 2 1)".to_string())
    );
}
