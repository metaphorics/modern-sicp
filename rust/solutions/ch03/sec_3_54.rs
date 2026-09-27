// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.54 and the edition addition
//! 3.54a: `mul-streams` is `add-streams` with multiplication in place of
//! addition, and `factorials` conses 1 onto `integers` times
//! `factorials` itself -- the k-th element of that product multiplies
//! the previous factorial by its 1-based index, so the stream reads the
//! factorials from 0! up, with the consed 1 serving as both 0! and 1!.
//! The addition builds the Catalan numbers the same way -- `C_0` = 1
//! consed onto the running recurrence over the stream's own past -- and
//! cross-checks them against the closed form computed from the
//! factorials stream itself.

use std::ops::Mul;

use ch03::sec_3_5::{
    Stream, cons_stream, integers, integers_starting_from, self_stream, stream_map2, stream_ref,
};

/// The book's `mul-streams`: the elementwise product of two streams.
/// Exercise-local: the section module ships only the additive twin.
fn mul_streams<A>(s1: &Stream<A>, s2: &Stream<A>) -> Stream<A>
where
    A: Mul<Output = A> + Copy + 'static,
{
    stream_map2(|a, b| *a * *b, s1, s2)
}

/// The book's `factorials`: 1 consed onto `integers` times
/// `factorials` -- the completed definition fills the statement's two
/// holes with `integers` first and the stream's own tail second, read
/// inside the tail thunk only.
#[must_use]
fn factorials() -> Stream<i128> {
    self_stream(|f| cons_stream(1, move || mul_streams(&integers(), &f.stream())))
}

/// The edition addition's Catalan stream: `C_0` = 1 consed onto the
/// running recurrence, each element multiplying the one at the same
/// position by 4n + 2 and dividing by n + 2, where n counts that
/// position from 0 -- hence the integers from 0 as the pairing stream.
/// The stream names itself, so every new element reads the one memoized
/// element before it.
#[must_use]
fn catalan() -> Stream<i128> {
    self_stream(|cat| {
        cons_stream(1, move || {
            stream_map2(
                |c, n| c * (4 * n + 2) / (n + 2),
                &cat.stream(),
                &integers_starting_from(0),
            )
        })
    })
}

/// The statement's closed form `C_n = (2n)! / (n! * (n + 1)!)` computed
/// from the factorials stream: each factorial is read by index off the
/// memoized spine, so repeated reads are cheap. The division is exact,
/// because the formula counts integer arrangements.
///
/// # Panics
/// Panics when `n > 16`, where `(2 * n)!` overflows `i128`; the pinned
/// range stops one step short, at 32!.
#[must_use]
fn closed_form_from_factorials(factorials: &Stream<i128>, n: usize) -> i128 {
    stream_ref(factorials, 2 * n) / (stream_ref(factorials, n) * stream_ref(factorials, n + 1))
}

/// The same closed form rearranged as `(n + 2) * ... * (2n) / n!`, the
/// evaluation that stays inside `i128` past where `(2n)!` overflows:
/// its intermediates peak near 1.6e28 at `n = 20`. The division is
/// exact, for the same counting reason as the direct form.
#[must_use]
fn closed_form_product(n: u32) -> i128 {
    let n = i128::from(n);
    ((n + 2)..=(2 * n)).product::<i128>() / (1..=n).product::<i128>()
}

mod ex_3_54 {
    use ch03::sec_3_5::stream_ref;

    /// Exercise 3.54: mul-streams and factorial stream
    ///
    /// Answers the first eight elements of `factorials` and the element
    /// at index 10 of the same stream.
    #[must_use]
    pub fn ex_3_54() -> (Vec<i128>, i128) {
        let stream = super::factorials();
        (stream.iter().take(8).collect(), stream_ref(&stream, 10))
    }
}

mod ex_3_54a {
    /// Edition addition 3.54a: stream of Catalan numbers
    ///
    /// Answers the first 21 elements of the self-referential Catalan
    /// stream, the statement's closed form read off the factorials
    /// stream for n = 0..=16 -- as far as i128 factorials reach -- and
    /// the overflow-safe form of the same formula through n = 20. All
    /// three must agree element for element.
    #[must_use]
    pub fn ex_3_54a() -> (Vec<i128>, Vec<i128>, Vec<i128>) {
        let facts = super::factorials();
        let from_stream: Vec<i128> = super::catalan().iter().take(21).collect();
        let closed = (0_usize..=16)
            .map(|n| super::closed_form_from_factorials(&facts, n))
            .collect();
        let extended = (0_u32..=20).map(super::closed_form_product).collect();
        (from_stream, closed, extended)
    }
}

#[test]
fn ex_3_54() {
    let (prefix, at_ten) = ex_3_54::ex_3_54();
    // factorials = 1 cons (integers * factorials): the head of the
    // product is 1 * 1, its next element is 2 * 1, then 3 * 2, 4 * 6,
    // ... -- each element multiplies the previous factorial by its
    // 1-based index.
    assert_eq!(prefix, vec![1, 1, 2, 6, 24, 120, 720, 5040]);
    // Element k of the stream is k!, so index 10 is 10!.
    assert_eq!(at_ten, 3_628_800);
}

#[test]
fn ex_3_54a() {
    let (from_stream, closed, extended) = ex_3_54a::ex_3_54a();
    // C_0 through C_10: the canonical Catalan numbers.
    assert_eq!(
        from_stream[..11],
        [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796]
    );
    // The recurrence's division is exact at every step: (n + 2) always
    // divides C_n * (4n + 2), because C_{n+1} = (2n+2)! / ((n+1)! *
    // (n+2)!) is an integer equal to exactly that quotient.
    let mut step = 1_i128;
    for n in 0_u32..20 {
        let numerator = step * (4 * i128::from(n) + 2);
        assert_eq!(numerator % (i128::from(n) + 2), 0);
        step = numerator / (i128::from(n) + 2);
    }
    // The exact-division walk lands on the stream's own element: had
    // the stream's integer division truncated anywhere, it would show
    // here.
    assert_eq!(step, from_stream[20]);
    // The stream agrees with the closed form read off the factorials
    // stream as far as i128 factorials reach, and with the
    // overflow-safe form of the same formula through n = 20.
    assert_eq!(from_stream[..17], closed[..]);
    assert_eq!(from_stream, extended);
}
