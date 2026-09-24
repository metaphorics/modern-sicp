// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.53: `s` names its own tail, and
//! that tail adds `s` to itself. Adding a stream to itself doubles every
//! element -- the two summands share the one memoized spine, so nothing
//! computes twice -- and the 1 consed at the front therefore grows by
//! doubling: the powers of two.

use ch03::sec_3_5::{Stream, add_streams, cons_stream, self_stream};

/// The book's `(define s (cons-stream 1 (add-streams s s)))`: the name
/// `s` is handed to the body and read inside the tail thunk only, which
/// runs after the definition is complete.
#[must_use]
fn doubling() -> Stream<i128> {
    self_stream(|s| cons_stream(1, move || add_streams(&s.stream(), &s.stream())))
}

mod ex_3_53 {
    /// Exercise 3.53: predict self-referential doubling stream
    ///
    /// Answers the first eight elements of `s`: 1 and then the powers of
    /// two, each element double the one before.
    #[must_use]
    pub fn ex_3_53() -> Vec<i128> {
        super::doubling().iter().take(8).collect()
    }
}

#[test]
fn ex_3_53() {
    let prefix = ex_3_53::ex_3_53();
    // The tail is s + s, and elementwise doubling sends each element to
    // its double: after the consed 1 the elements are 2, 4, 8, ... -- by
    // induction s[k] = 2^k, the powers of two.
    assert_eq!(prefix, vec![1, 2, 4, 8, 16, 32, 64, 128]);
}
