// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.50: the book's `stream-map`
//! completed for any number of streams. The variadic `(proc .
//! argstreams)` definition has two holes -- the emptiness test on the
//! car of the argument list and the constructor -- and the general
//! answer reads them over the whole list: the map is empty as soon as
//! ANY input stream is, and otherwise one call of the procedure combines
//! all the heads while the tail maps the procedure over the forced
//! tails.

use ch03::sec_3_5::{Stream, add_streams, cons_stream, integers};

/// The book's general `stream-map` over a list of streams: one call of
/// `f` combines the heads of the nonempty inputs, the tail maps `f` over
/// the tails, and the result is empty as soon as any input is.
/// Exercise-local: the section module ships the one- and two-stream
/// forms this completes.
pub fn stream_map_n<A, B, F>(f: F, streams: &[Stream<A>]) -> Stream<B>
where
    A: Clone + 'static,
    B: 'static,
    F: Fn(&[A]) -> B + Clone + 'static,
{
    if streams.iter().any(Stream::is_empty) {
        return Stream::Empty;
    }
    let heads: Vec<A> = streams.iter().map(Stream::head).cloned().collect();
    let head = f(&heads);
    let sources: Vec<Stream<A>> = streams.to_vec();
    cons_stream(head, move || {
        let tails: Vec<Stream<A>> = sources.iter().map(Stream::tail).collect();
        stream_map_n(f, &tails)
    })
}

mod ex_3_50 {
    use ch03::sec_3_5::{integers, ones, stream_enumerate_interval};

    use super::stream_map_n;
    /// Exercise 3.50: multi-stream map completion
    ///
    /// Answers three pinned prefixes of the general map: the elementwise
    /// sum of two `integers` streams, which must agree with
    /// `add_streams`; the product of `integers`, `ones`, and
    /// `integers`; and the sum of `integers` with a three-element finite
    /// stream.
    #[must_use]
    pub fn ex_3_50() -> (Vec<i128>, Vec<i128>, Vec<i128>) {
        let summed = stream_map_n(|xs| xs[0] + xs[1], &[integers(), integers()]);
        let multiplied = stream_map_n(
            |xs| xs[0] * xs[1] * xs[2],
            &[integers(), ones(), integers()],
        );
        let short = stream_map_n(
            |xs| xs[0] + xs[1],
            &[integers(), stream_enumerate_interval(1, 3)],
        );
        (
            summed.iter().take(6).collect(),
            multiplied.iter().take(5).collect(),
            short.iter().take(6).collect(),
        )
    }
}

#[test]
fn ex_3_50() {
    let (two_summed, three_multiplied, short_empties) = ex_3_50::ex_3_50();
    // (integers + integers)[k] = 2(k + 1): the completed general map
    // must reproduce the two-stream `add-streams` over the prefix.
    assert_eq!(
        two_summed,
        add_streams(&integers(), &integers())
            .iter()
            .take(6)
            .collect::<Vec<i128>>()
    );
    // integers * ones * integers = n * 1 * n: the head call sees all
    // three heads at once, as `apply proc` does in the book.
    assert_eq!(three_multiplied, vec![1, 4, 9, 16, 25]);
    // One empty input empties the whole map: the finite stream runs out
    // at three elements, so the result ends there too.
    assert_eq!(short_empties, vec![2, 4, 6]);
}
