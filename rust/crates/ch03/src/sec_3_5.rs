// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.5

//! Section 3.5: Streams.
//!
//! The section's programs live here as free functions over the
//! runtime's [`Stream`] and memoized [`Lazy`]. The empty stream is
//! [`Stream::Empty`] and [`Stream::is_empty`](Stream::is_empty) tests
//! for it; [`cons_stream`] takes the tail as a thunk; [`delay`] and
//! [`force`] are backed by the one memoized thunk of this edition. A stream's `Clone` shares the memoized spine, so two
//! references to one stream force each tail at most once -- the fact
//! exercises 3.51, 3.52, 3.57, and 3.63 measure.
//!
//! Many section streams are self-referential: the stream is named
//! inside its own tail. Rust definitions cannot name themselves, so
//! [`self_stream`] hands the body a [`StreamName`]: call
//! `name.stream()` inside tail thunks only, which run after the
//! definition is complete. Self-referential streams form an `Rc` cycle
//! and leak by design, as the runtime stream documents.
//!
//! Nothing here prints: the display procedures answer the lines the
//! book's procedures print, and the example programs print them.

use std::cell::RefCell;
use std::fmt::Display;
use std::ops::{Add, Mul};
use std::rc::Rc;

pub use sicp_runtime::{Lazy, Stream};

pub use crate::sec_3_1::{RANDOM_INIT, gcd, rand_update};

/// The book's `the-empty-stream`.
#[must_use]
pub fn the_empty_stream<A>() -> Stream<A> {
    Stream::Empty
}

/// Conses an eager head onto a thunked tail that runs at most once.
/// The tail is a thunk because a plain argument would be evaluated
/// before the call, which this constructor exists to prevent.
pub fn cons_stream<A: 'static>(head: A, tail: impl FnOnce() -> Stream<A> + 'static) -> Stream<A> {
    Stream::cons_stream(head, tail)
}

/// The book's `stream-car`.
///
/// # Panics
/// Panics on the empty stream, as `stream-car` of `the-empty-stream` is
/// an error in the book.
#[must_use]
pub fn stream_head<A: 'static>(s: &Stream<A>) -> &A {
    s.head()
}

/// The book's `stream-cdr`: forces and returns the memoized tail.
///
/// # Panics
/// Panics on the empty stream, as `stream-cdr` of `the-empty-stream` is
/// an error in the book.
#[must_use]
pub fn stream_tail<A: Clone + 'static>(s: &Stream<A>) -> Stream<A> {
    s.tail()
}

/// The one memoized promise of this edition.
pub fn delay<T: 'static, F: FnOnce() -> T + 'static>(f: F) -> Lazy<T> {
    Lazy::new(f)
}

/// Runs the generator once, then serves the
/// memoized value forever.
#[must_use]
pub fn force<T: 'static>(promise: &Lazy<T>) -> Rc<T> {
    promise.force()
}

/// Selects the `n`-th element, counting from 0: the book's
/// `stream-ref`.
///
/// # Panics
/// Panics when the stream ends before index `n`, which is how the book's
/// `stream-ref` reports an index past the end.
#[must_use]
pub fn stream_ref<A: Clone + 'static>(s: &Stream<A>, n: usize) -> A {
    if n == 0 {
        return s.head().clone();
    }
    stream_ref(&s.tail(), n - 1)
}

/// The book's `stream-map` over one stream.
pub fn stream_map<A, B, F>(f: F, s: &Stream<A>) -> Stream<B>
where
    A: Clone + 'static,
    B: 'static,
    F: Fn(&A) -> B + Clone + 'static,
{
    if s.is_empty() {
        return Stream::Empty;
    }
    let head = f(s.head());
    let source = s.clone();
    cons_stream(head, move || stream_map(f, &source.tail()))
}

/// The book's `stream-map` over two streams, the form `add-streams`
/// needs; exercise 3.50 completes the general version over a list of
/// streams.
pub fn stream_map2<A, B, C, F>(f: F, s1: &Stream<A>, s2: &Stream<B>) -> Stream<C>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
    F: Fn(&A, &B) -> C + Clone + 'static,
{
    if s1.is_empty() || s2.is_empty() {
        return Stream::Empty;
    }
    let head = f(s1.head(), s2.head());
    let first = s1.clone();
    let second = s2.clone();
    cons_stream(head, move || stream_map2(f, &first.tail(), &second.tail()))
}

/// The book's `stream-for-each`, walking the whole stream and applying
/// `proc` to every element.
pub fn stream_for_each<A, F>(mut proc: F, s: &Stream<A>)
where
    A: Clone + 'static,
    F: FnMut(&A),
{
    if s.is_empty() {
        return;
    }
    proc(s.head());
    stream_for_each(proc, &s.tail());
}

/// The book's `display-line`, answering the line it would print.
#[must_use]
pub fn display_line<A: Display>(x: &A) -> String {
    format!("{x}")
}

/// The book's `display-stream`, answering the lines it would print, one
/// per element in order; the example programs print them.
#[must_use]
pub fn display_stream<A>(s: &Stream<A>) -> Vec<String>
where
    A: Display + Clone + 'static,
{
    let mut lines = Vec::new();
    stream_for_each(|x| lines.push(format!("{x}")), s);
    lines
}

/// The book's `show` of exercise 3.51: records the element at the moment
/// it is computed and hands it back unchanged, so the log tells when the
/// delay let it run.
pub fn show<A: Display>(x: A, log: &Rc<RefCell<Vec<String>>>) -> A {
    log.borrow_mut().push(format!("{x}"));
    x
}

/// The book's `stream-filter`: the head of the first element that
/// satisfies `pred`, and the filtered rest delayed behind it.
pub fn stream_filter<A, F>(pred: F, s: &Stream<A>) -> Stream<A>
where
    A: Clone + 'static,
    F: Fn(&A) -> bool + Clone + 'static,
{
    if s.is_empty() {
        return Stream::Empty;
    }
    let head = s.head().clone();
    if pred(&head) {
        let source = s.clone();
        return cons_stream(head, move || stream_filter(pred, &source.tail()));
    }
    stream_filter(pred, &s.tail())
}

/// The book's `stream-enumerate-interval` over exact integers.
#[must_use]
pub fn stream_enumerate_interval(low: i128, high: i128) -> Stream<i128> {
    if low > high {
        return Stream::Empty;
    }
    cons_stream(low, move || stream_enumerate_interval(low + 1, high))
}

/// The binding site of a self-referential definition, where the
/// stream is named inside its own tail. [`self_stream`] hands the body one of these; call
/// [`StreamName::stream`] inside tail thunks only, which run after the
/// definition is complete.
pub struct StreamName<A>(Rc<dyn Fn() -> Stream<A>>);

impl<A> Clone for StreamName<A> {
    fn clone(&self) -> Self {
        StreamName(Rc::clone(&self.0))
    }
}

impl<A: 'static> StreamName<A> {
    /// The stream being defined; every call answers a clone sharing the
    /// one memoized spine.
    ///
    /// # Panics
    /// Panics when called before the definition is complete, which is the
    /// eager use the book's `define` also cannot serve.
    #[must_use]
    pub fn stream(&self) -> Stream<A> {
        (self.0)()
    }
}

/// Defines a stream whose own tail may name the whole: the book's
/// self-referential `define`. The body builds the first cell(s), calling
/// `name.stream()` inside its tail thunks; when the body returns, the
/// name is bound, so every later force sees the finished stream.
///
/// # Panics
/// Panics when the body calls `name.stream()` eagerly, before the
/// definition is complete: the book's `define` serves the same rule by
/// looping or failing on such a use.
pub fn self_stream<A: Clone + 'static>(body: impl FnOnce(StreamName<A>) -> Stream<A>) -> Stream<A> {
    let slot: Rc<RefCell<Option<Stream<A>>>> = Rc::new(RefCell::new(None));
    let name = {
        let slot = Rc::clone(&slot);
        StreamName(Rc::new(move || {
            slot.borrow()
                .as_ref()
                .expect("stream name read before its definition finished")
                .clone()
        }))
    };
    let defined = body(name);
    *slot.borrow_mut() = Some(defined.clone());
    defined
}

/// The integers from `n` upward.
#[must_use]
pub fn integers_starting_from(n: i128) -> Stream<i128> {
    cons_stream(n, move || integers_starting_from(n + 1))
}

/// The book's `no-sevens`: the integers not divisible by 7.
#[must_use]
pub fn no_sevens() -> Stream<i128> {
    stream_filter(|x| x % 7 != 0, &integers_starting_from(1))
}

/// The book's `ones`: the infinite stream of 1s, defined in terms of
/// itself.
#[must_use]
pub fn ones() -> Stream<i128> {
    self_stream(|ones| cons_stream(1, move || ones.stream()))
}

/// The book's `add-streams`: the elementwise sum of two number streams.
#[must_use]
pub fn add_streams<A>(s1: &Stream<A>, s2: &Stream<A>) -> Stream<A>
where
    A: Add<Output = A> + Copy + 'static,
{
    stream_map2(|a, b| *a + *b, s1, s2)
}

/// The book's `integers`: 1, 2, 3, ..., built as `integers` plus `ones`.
#[must_use]
pub fn integers() -> Stream<i128> {
    self_stream(|integers| {
        let unit = ones();
        cons_stream(1, move || add_streams(&integers.stream(), &unit))
    })
}

/// The book's `fibgen`: `a`, `b`, `a+b`, ....
#[must_use]
pub fn fibgen(a: i128, b: i128) -> Stream<i128> {
    cons_stream(a, move || fibgen(b, a + b))
}

/// The book's `fibs`: the Fibonacci stream defined by adding `fibs` to
/// its own tail.
#[must_use]
pub fn fibs() -> Stream<i128> {
    self_stream(|fibs| {
        cons_stream(0, move || {
            cons_stream(1, {
                let named = fibs.clone();
                move || add_streams(&named.stream().tail(), &named.stream())
            })
        })
    })
}

/// The book's `scale-stream`: every element times `factor`.
#[must_use]
pub fn scale_stream<A>(s: &Stream<A>, factor: A) -> Stream<A>
where
    A: Mul<Output = A> + Copy + 'static,
{
    stream_map(move |x| *x * factor, s)
}

/// The book's `sieve`: the first element, then the sieve of the stream
/// with all its multiples removed.
///
/// # Panics
/// Panics on the empty stream, which the sieve never reaches: it is
/// started on an infinite stream.
#[must_use]
pub fn sieve(s: &Stream<i128>) -> Stream<i128> {
    let first = *s.head();
    let source = s.clone();
    cons_stream(first, move || {
        sieve(&stream_filter(move |x| x % first != 0, &source.tail()))
    })
}

/// The book's `primes` of 3.5.2: the sieve run over the integers from 2.
#[must_use]
pub fn primes() -> Stream<i128> {
    sieve(&integers_starting_from(2))
}

/// Tests `n` for primality by trial division over a generated primes
/// stream, stopping once `p*p > n`: the book's `prime?` of the
/// alternative primes definition at the end of 3.5.2.
#[must_use]
pub fn is_prime(n: i128, primes: &Stream<i128>) -> bool {
    let mut ps = primes.clone();
    loop {
        let p = *ps.head();
        if p * p > n {
            return true;
        }
        if n % p == 0 {
            return false;
        }
        ps = ps.tail();
    }
}

/// The book's alternative `primes`: 2 consed to start, then the integers
/// from 3 filtered by the `prime?` that tests against this same primes
/// stream, one self-reference inside the other. Enough of the stream is
/// always generated to test the next candidate, because `prime?` stops
/// at the square root.
#[must_use]
pub fn primes_filtered() -> Stream<i128> {
    self_stream(|primes| {
        cons_stream(2, {
            let named = primes.clone();
            move || {
                stream_filter(
                    move |n| is_prime(*n, &named.stream()),
                    &integers_starting_from(3),
                )
            }
        })
    })
}

/// The book's `partial-sums` of exercise 3.55, used by the text's
/// pi approximations: running totals of `s`.
///
/// # Panics
/// Panics on the empty stream, whose partial sums have no first element.
#[must_use]
pub fn partial_sums<A>(s: &Stream<A>) -> Stream<A>
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

/// The book's `sqrt-improve`: one Newton step for the square root of
/// `x` from `guess`.
#[must_use]
#[expect(
    clippy::manual_midpoint,
    reason = "the book's averaging formula stays as written"
)]
pub fn sqrt_improve(guess: f64, x: f64) -> f64 {
    (guess + x / guess) / 2.0
}

/// The book's `sqrt-stream` of 3.5.3: the Newton approximations to the
/// square root of `x`, each computed from the one before, defined in
/// terms of itself.
#[must_use]
pub fn sqrt_stream(x: f64) -> Stream<f64> {
    self_stream(move |guesses| {
        cons_stream(1.0, move || {
            stream_map(move |g| sqrt_improve(*g, x), &guesses.stream())
        })
    })
}

/// The book's `pi-summands`: 1, -1/3, 1/5, ..., the alternating series
/// whose partial sums times 4 approach pi.
#[must_use]
pub fn pi_summands(n: f64) -> Stream<f64> {
    cons_stream(1.0 / n, move || stream_map(|x| -*x, &pi_summands(n + 2.0)))
}

/// The book's `pi-stream`: four times the partial sums of
/// [`pi_summands`].
#[must_use]
pub fn pi_stream() -> Stream<f64> {
    scale_stream(&partial_sums(&pi_summands(1.0)), 4.0)
}

/// The book's `euler-transform`: one sequence-acceleration step over the
/// first three elements, with the rest delayed.
///
/// # Panics
/// Panics when the stream has fewer than three elements, which the
/// text's approximating streams never do.
#[must_use]
pub fn euler_transform(s: &Stream<f64>) -> Stream<f64> {
    let s0 = stream_ref(s, 0);
    let s1 = stream_ref(s, 1);
    let s2 = stream_ref(s, 2);
    let head = s2 - (s2 - s1) * (s2 - s1) / (s0 + -2.0 * s1 + s2);
    let source = s.clone();
    cons_stream(head, move || euler_transform(&source.tail()))
}

/// The book's `make-tableau`: a stream of streams where each new row is
/// `transform` applied to the row before.
pub fn make_tableau<F>(transform: F, s: &Stream<f64>) -> Stream<Stream<f64>>
where
    F: Fn(&Stream<f64>) -> Stream<f64> + Clone + 'static,
{
    let source = s.clone();
    cons_stream(source.clone(), move || {
        make_tableau(transform.clone(), &transform(&source))
    })
}

/// The book's `accelerated-sequence`: the first element of every row of
/// [`make_tableau`], the recursive accelerator.
#[must_use]
pub fn accelerate_sequence<F>(transform: F, s: &Stream<f64>) -> Stream<f64>
where
    F: Fn(&Stream<f64>) -> Stream<f64> + Clone + 'static,
{
    stream_map(|row| *row.head(), &make_tableau(transform, s))
}

/// The book's `interleave`: alternates elements from `s1` and `s2`, so
/// even an infinite `s1` cannot starve `s2`.
pub fn interleave<A: Clone + 'static>(s1: &Stream<A>, s2: &Stream<A>) -> Stream<A> {
    if s1.is_empty() {
        return s2.clone();
    }
    let head = s1.head().clone();
    let front = s1.clone();
    let back = s2.clone();
    cons_stream(head, move || interleave(&back, &front.tail()))
}

/// The book's `pairs` of 3.5.3: the pairs `(i, j)` with `i` from `s` and
/// `j` from `t`, ordered by the `interleave` decomposition that keeps
/// every pair reachable on infinite inputs.
pub fn pairs<A: Clone + 'static>(s: &Stream<A>, t: &Stream<A>) -> Stream<(A, A)> {
    let head_pair = (s.head().clone(), t.head().clone());
    let named_head = s.head().clone();
    let mapped = stream_map(move |x| (named_head.clone(), x.clone()), &t.tail());
    let front = s.clone();
    let back = t.clone();
    cons_stream(head_pair, move || {
        interleave(&mapped, &pairs(&front.tail(), &back.tail()))
    })
}

/// The book's `merge` of exercise 3.56's discussion: the ordered union
/// of two ordered streams, with duplicates from both sides kept once.
pub fn merge<A: PartialOrd + Copy + 'static>(s1: &Stream<A>, s2: &Stream<A>) -> Stream<A> {
    if s1.is_empty() {
        return s2.clone();
    }
    if s2.is_empty() {
        return s1.clone();
    }
    let x1 = *s1.head();
    let x2 = *s2.head();
    let front = s1.clone();
    let back = s2.clone();
    if x1 < x2 {
        cons_stream(x1, move || merge(&front.tail(), &back))
    } else if x2 < x1 {
        cons_stream(x2, move || merge(&front, &back.tail()))
    } else {
        cons_stream(x1, move || merge(&front.tail(), &back.tail()))
    }
}

/// The book's `integral` of 3.5.3: the implicit integrator whose output
/// `int` is defined in terms of itself, the feedback loop of
/// Figure 3.32.
#[must_use]
pub fn integral(integrand: &Stream<f64>, initial_value: f64, dt: f64) -> Stream<f64> {
    self_stream(move |int| {
        let source = integrand.clone();
        cons_stream(initial_value, move || {
            add_streams(&scale_stream(&source, dt), &int.stream())
        })
    })
}

/// The book's `sign-change-detector` of 3.5.3: 1 when the signal crossed
/// from negative `previous` to positive `current`, -1 for the crossing
/// down, 0 otherwise.
#[must_use]
pub fn sign_change_detector(current: f64, previous: f64) -> f64 {
    if previous < 0.0 && current > 0.0 {
        1.0
    } else if previous > 0.0 && current < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// The book's `make-zero-crossings` of 3.5.3: the sense-data stream run
/// through [`sign_change_detector`], each output paired with the value
/// before it.
///
/// # Panics
/// Panics on the empty stream, whose crossings cannot be defined.
#[must_use]
pub fn make_zero_crossings(input_stream: &Stream<f64>, last_value: f64) -> Stream<f64> {
    let source = input_stream.clone();
    let head = sign_change_detector(*source.head(), last_value);
    cons_stream(head, move || {
        make_zero_crossings(&source.tail(), *source.head())
    })
}

/// The book's `integral` of 3.5.4: the integrand arrives as a delayed
/// argument, so feedback systems such as [`solve`] can be built even
/// though the integrand's first element needs the answer's first
/// element.
pub fn integral_delayed<F>(delayed_integrand: F, initial_value: f64, dt: f64) -> Stream<f64>
where
    F: FnOnce() -> Stream<f64> + 'static,
{
    self_stream(move |int| {
        cons_stream(initial_value, move || {
            let integrand = delayed_integrand();
            add_streams(&scale_stream(&integrand, dt), &int.stream())
        })
    })
}

/// The book's `solve` of 3.5.4: the solution of `dy/dt = f(y)` with
/// `y(0) = y0`, integrated at step `dt`, built by delaying `dy` until
/// `integral` asks for it.
pub fn solve<F>(f: F, y0: f64, dt: f64) -> Stream<f64>
where
    F: Fn(f64) -> f64 + Clone + 'static,
{
    self_stream(move |y| {
        let dy = {
            let named = y.clone();
            move || stream_map(move |yv| f(*yv), &named.stream())
        };
        integral_delayed(dy, y0, dt)
    })
}

/// The book's `random-numbers` of 3.5.5: the seeded generator's successive
/// words, defined by mapping `rand-update` over the stream itself, with
/// no assignment anywhere.
#[must_use]
pub fn random_numbers() -> Stream<u64> {
    self_stream(|randoms| {
        cons_stream(RANDOM_INIT, move || {
            stream_map(|x| rand_update(*x), &randoms.stream())
        })
    })
}

/// The book's `map-successive-pairs`: `f` over each consecutive pair of
/// `s`, consuming two elements per output.
pub fn map_successive_pairs<A, B, F>(f: F, s: &Stream<A>) -> Stream<B>
where
    A: Clone + 'static,
    B: 'static,
    F: Fn(&A, &A) -> B + Clone + 'static,
{
    let first = s.head().clone();
    let second = s.tail().head().clone();
    let source = s.clone();
    cons_stream(f(&first, &second), move || {
        map_successive_pairs(f, &source.tail().tail())
    })
}

/// The book's `cesaro-stream`: the Cesaro experiment on consecutive
/// pairs of [`random_numbers`], true when the pair is coprime.
#[must_use]
pub fn cesaro_stream() -> Stream<bool> {
    map_successive_pairs(|r1, r2| gcd(*r1, *r2) == 1, &random_numbers())
}

/// The book's `monte-carlo` of 3.5.5: a stream of running estimates of
/// the experiment's probability, one per trial, with no state but the
/// two counters carried forward.
#[must_use]
pub fn monte_carlo_stream(experiments: &Stream<bool>, passed: u32, failed: u32) -> Stream<f64> {
    let source = experiments.clone();
    let (pass, fail) = if *experiments.head() {
        (passed + 1, failed)
    } else {
        (passed, failed + 1)
    };
    cons_stream(f64::from(pass) / f64::from(pass + fail), move || {
        monte_carlo_stream(&source.tail(), pass, fail)
    })
}

/// The book's `pi` of 3.5.5: the square-root-of-6-over-probability map
/// over the Cesaro Monte Carlo run, a stream of ever-better estimates.
#[must_use]
pub fn pi_estimates() -> Stream<f64> {
    stream_map(
        |p| (6.0 / *p).sqrt(),
        &monte_carlo_stream(&cesaro_stream(), 0, 0),
    )
}

/// The book's `stream-withdraw` of 3.5.5: the balance history of a
/// withdrawal processor fed a stream of amounts, a mathematical function
/// with the behavior of the object of 3.1.3 and none of its state.
#[must_use]
pub fn stream_withdraw(balance: i128, amounts: &Stream<i128>) -> Stream<i128> {
    let source = amounts.clone();
    cons_stream(balance, move || {
        stream_withdraw(balance - *source.head(), &source.tail())
    })
}
