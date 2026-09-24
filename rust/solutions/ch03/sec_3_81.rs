// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.81: exercise 3.6's resettable
//! random-number generator as a stream transformation. The generator
//! is a fold over the request stream carrying one word of state: a
//! `Generate` request answers `rand-update` of the state and advances
//! it; a `Reset(n)` request answers `n` itself and makes it the state.
//! No assignment anywhere -- the "current word" exists only as the
//! argument each recursive call hands the next, so the answer stream
//! is a mathematical function of the request stream, and running the
//! same requests twice replays the same words.

use ch03::sec_3_1::{RANDOM_INIT, rand_update};
use ch03::sec_3_5::{Stream, cons_stream, random_numbers, stream_map, stream_ref};

/// The requests the generator answers: the book's `generate` symbol,
/// and the book's `reset` carrying the word to restart the sequence
/// from.
#[derive(Clone, Debug)]
pub enum RandRequest {
    /// Ask for the next word: [`rand_update`] of the current state.
    Generate,
    /// Restart the sequence: the answer is `n` itself, and `n` becomes
    /// the state.
    Reset(u64),
}

/// The book's resettable `rand` of exercise 3.6 as a stream: folds the
/// requests into the stream of answered words, the state starting at
/// `initial`.
///
/// # Panics
/// Panics when the request stream ends: the head of the request stream
/// must always hold the next request, so the generator is fed an
/// endless request stream, exactly as the book's version consumes an
/// endless experiment stream.
#[must_use]
pub fn rand_stream(requests: &Stream<RandRequest>, initial: u64) -> Stream<u64> {
    answer(requests, initial)
}

/// One fold step: answers the word the requests' head demands from the
/// state `state`, and pairs it with the generator fed the requests'
/// tail from the new state.
///
/// # Panics
/// Panics on the empty request stream, as [`rand_stream`] documents.
fn answer(requests: &Stream<RandRequest>, state: u64) -> Stream<u64> {
    let produced = match requests.head() {
        RandRequest::Generate => rand_update(state),
        RandRequest::Reset(n) => *n,
    };
    let rest = requests.clone();
    cons_stream(produced, move || answer(&rest.tail(), produced))
}

/// The exercise's script as a request stream, requests in order.
///
/// # Panics
/// Panics on an empty script, which no call site has: the scripts are
/// constants a few lines away.
fn script_stream(script: &[RandRequest]) -> Stream<RandRequest> {
    let (first, rest) = script.split_first().expect("non-empty script");
    let rest = rest.to_vec();
    cons_stream(first.clone(), move || script_stream(&rest))
}

/// The exercise's mixed script: two draws, a reset to 7, one draw.
fn mixed_script() -> Vec<RandRequest> {
    vec![
        RandRequest::Generate,
        RandRequest::Generate,
        RandRequest::Reset(7),
        RandRequest::Generate,
    ]
}

/// The endless `generate` request stream: every request asks for the
/// next word.
#[must_use]
fn generates() -> Stream<RandRequest> {
    cons_stream(RandRequest::Generate, generates)
}

mod ex_3_81 {
    use super::{RANDOM_INIT, mixed_script, rand_stream, script_stream, stream_ref};

    /// Exercise 3.81: rand request stream, no assignment
    ///
    /// Answers the words the mixed script draws from the edition's
    /// seeded generator (`random-init` = 1): `rand-update` of the
    /// running word, `rand-update` again, then the reset value 7
    /// itself, then `rand-update` of 7.
    #[must_use]
    pub fn ex_3_81() -> [u64; 4] {
        let requests = script_stream(&mixed_script());
        let answers = rand_stream(&requests, RANDOM_INIT);
        std::array::from_fn(|i| stream_ref(&answers, i))
    }
}

#[test]
fn ex_3_81() {
    // Hand values of the seeded chain: rand-update of 1, then
    // rand-update of that word, then the reset answers 7 itself, then
    // rand-update of 7 -- each exactly `rand-update` applied to the
    // state the previous step left.
    assert_eq!(
        ex_3_81::ex_3_81(),
        [
            5_180_492_295_206_395_165,
            2_586_950_713_725_923_525,
            7,
            15_130_880_334_998_875_822,
        ]
    );
}

/// One request stream run twice answers element-identical words: with
/// no hidden word of state there is nothing to drift, which is the
/// repeatability exercise 3.6 wanted from the reset.
#[test]
fn same_requests_replay_identically() {
    let requests = script_stream(&mixed_script());
    let first = rand_stream(&requests, RANDOM_INIT);
    let second = rand_stream(&requests, RANDOM_INIT);
    assert!((0..4).all(|i| stream_ref(&first, i) == stream_ref(&second, i)));
}

/// The all-generate request stream from the edition seed answers
/// exactly `rand-update` mapped over the book's `random-numbers`: the
/// request formulation and the self-referential one of 3.5.5 are the
/// same mathematical function, both grown from the fixed seed.
#[test]
fn generate_only_matches_the_books_random_numbers() {
    let answers = rand_stream(&generates(), RANDOM_INIT);
    let shifted = stream_map(|x| rand_update(*x), &random_numbers());
    assert!((0..6).all(|i| stream_ref(&answers, i) == stream_ref(&shifted, i)));
    assert_eq!(stream_ref(&answers, 0), 5_180_492_295_206_395_165);
    assert_eq!(stream_ref(&answers, 1), 2_586_950_713_725_923_525);
    assert_eq!(stream_ref(&answers, 2), 3_968_523_955_086_520_050);
}
