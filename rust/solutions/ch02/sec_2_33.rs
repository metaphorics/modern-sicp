// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.33, one module and one test.

mod ex_2_33 {
    use ch02::sec_2_2::{List, accumulate};

    /// Exercise 2.33: `map` as an accumulation: cons the transformed
    /// element onto the accumulation of the rest, so the answer is
    /// built back to front from the far end.
    fn map_acc<T, U: Clone>(p: impl Fn(&T) -> U, sequence: &[T]) -> List<U> {
        accumulate(
            |x, acc: List<U>| List::cons(p(x), &acc),
            List::Nil,
            sequence,
        )
    }

    /// Exercise 2.33: `append` as an accumulation with `cons`: the
    /// initial value is the whole second sequence, and consing the
    /// first sequence's elements onto it, right to left, builds the
    /// combined list.
    fn append_acc(seq1: &[i128], seq2: &List<i128>) -> List<i128> {
        accumulate(
            |x, acc: List<i128>| List::cons(*x, &acc),
            seq2.clone(),
            seq1,
        )
    }

    /// Exercise 2.33: `length` as an accumulation: every element
    /// contributes one to the count accumulated so far.
    fn length_acc<T>(sequence: &[T]) -> usize {
        accumulate(|_, n| n + 1, 0, sequence)
    }

    /// Exercise 2.33: list operations as accumulations
    ///
    /// Returns the rendered `map` of squaring over `(1 2 3 4)`, the
    /// rendered `append` of `(1 2 3)` and `(4 5)`, and the `length` of
    /// `(1 2 3)`, all built on `accumulate`, in that order.
    pub fn ex_2_33() -> (String, String, usize) {
        let mapped = map_acc(|x: &i128| x * x, &[1, 2, 3, 4]);
        let appended = append_acc(&[1, 2, 3], &List::from_iter([4, 5]));
        (
            mapped.to_string(),
            appended.to_string(),
            length_acc(&[1, 2, 3]),
        )
    }
}

#[test]
fn ex_2_33() {
    assert_eq!(
        ex_2_33::ex_2_33(),
        ("(1 4 9 16)".to_string(), "(1 2 3 4 5)".to_string(), 3)
    );
}
