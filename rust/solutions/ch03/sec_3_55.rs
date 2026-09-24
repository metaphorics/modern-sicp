// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.55: `partial-sums` answers S0,
//! then the running sums -- which is the stream of sums added, element
//! by element, to the tail of S. Defining it through one self-reference
//! makes each new total one addition past the memoized one before it,
//! and the local definition must agree with the section module's own
//! `partial_sums`, the combinator behind the text's pi stream.

use std::ops::Add;

use ch03::sec_3_5::{Stream, add_streams, cons_stream, self_stream};

/// The book's `partial-sums` of exercise 3.55, defined locally as the
/// statement asks: the first element of `s`, then `sums` added to the
/// tail of `s`. Exercise-local: the section module already ships this
/// combinator, and the test cross-checks the two over a prefix.
fn partial_sums_local<A>(s: &Stream<A>) -> Stream<A>
where
    A: Add<Output = A> + Copy + 'static,
{
    self_stream(|sums| {
        let source = s.clone();
        cons_stream(*s.head(), move || {
            add_streams(&sums.stream(), &source.tail())
        })
    })
}

mod ex_3_55 {
    use ch03::sec_3_5::{integers, partial_sums};

    /// Exercise 3.55: partial-sums combinator
    ///
    /// Answers the first five partial sums of `integers` -- the
    /// statement's example stream -- from the local definition and from
    /// the section's `partial_sums`.
    #[must_use]
    pub fn ex_3_55() -> (Vec<i128>, Vec<i128>) {
        let local = super::partial_sums_local(&integers());
        let library = partial_sums(&integers());
        (
            local.iter().take(5).collect(),
            library.iter().take(5).collect(),
        )
    }
}

#[test]
fn ex_3_55() {
    let (local, library) = ex_3_55::ex_3_55();
    // The n-th partial sum of the integers is the triangular number
    // 1 + ... + (n + 1): exactly the statement's example.
    assert_eq!(local, vec![1, 3, 6, 10, 15]);
    // The section module uses the same combinator for the pi stream;
    // the local definition must agree with it element for element.
    assert_eq!(library, vec![1, 3, 6, 10, 15]);
}
