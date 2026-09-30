// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.63: why the book's
//! `sqrt-stream` keeps `guesses` as a local variable. The module's
//! version defines the Newton approximations once, self-referentially,
//! so every reference shares one memoized spine and producing element
//! `k` costs exactly `k` calls of `sqrt-improve`. Louis's version conses
//! the first guess onto a map whose source is a fresh recursive call,
//! so nothing is shared between the levels of the tower: producing
//! element `k` re-derives a prefix of length `k` across the fresh
//! levels, costing `1 + 2 + ... + k` improve calls in total. Both
//! structures are run here with an `Rc<Cell<u32>>` meter wrapped around
//! the improve step, and the counts are pinned from those runs.

use std::cell::Cell;
use std::rc::Rc;

use ch03::sec_3_5::{Stream, cons_stream, self_stream, sqrt_improve, sqrt_stream, stream_map};

/// One Newton improve step under a meter: increments the meter once
/// per improve it performs, then answers the book's `sqrt-improve`.
fn metered_improve(x: f64, meter: &Rc<Cell<u32>>) -> impl Fn(&f64) -> f64 + Clone + 'static {
    let meter = Rc::clone(meter);
    move |g: &f64| {
        meter.set(meter.get() + 1);
        sqrt_improve(*g, x)
    }
}

/// The book's `sqrt-stream` with the meter in place of the bare
/// improve step. The delay structure is the module's exactly -- one
/// self-referential definition whose tail maps over the named stream --
/// so the meter sees exactly the improves the shared version performs.
fn counting_shared_sqrt_stream(x: f64, improves: &Rc<Cell<u32>>) -> Stream<f64> {
    let step = metered_improve(x, improves);
    self_stream(move |guesses| {
        cons_stream(1.0, move || stream_map(step.clone(), &guesses.stream()))
    })
}

/// Louis's `sqrt-stream` with the same meter: the tail maps the
/// improvement over a fresh recursive call, so each tail force builds a
/// brand-new stream at the next level of the tower and no memo is ever
/// shared between levels.
fn counting_louis_sqrt_stream(x: f64, improves: &Rc<Cell<u32>>) -> Stream<f64> {
    let meter = Rc::clone(improves);
    cons_stream(1.0, move || {
        stream_map(
            metered_improve(x, &meter),
            &counting_louis_sqrt_stream(x, &meter),
        )
    })
}

/// Produces the first `elements` elements of the counted stream and
/// answers the meter along with the values produced, asserting on the
/// way that the metered twin computes the same Newton iterates as the
/// module's `sqrt_stream` -- the meter is the only difference.
#[must_use]
fn checked_prefix(
    counted: &Stream<f64>,
    x: f64,
    elements: usize,
    improves: &Rc<Cell<u32>>,
) -> (u32, Vec<f64>) {
    let values: Vec<f64> = counted.iter().take(elements).collect();
    let module_values: Vec<f64> = sqrt_stream(x).iter().take(elements).collect();
    let agrees = values
        .iter()
        .zip(module_values.iter())
        .all(|(a, b)| (a - b).abs() < f64::EPSILON);
    assert!(agrees, "metered twin diverged from the module's stream");
    (improves.get(), values)
}

/// The shared version's prefix of `elements` elements for `x`, metered.
#[must_use]
fn shared_prefix(x: f64, elements: usize) -> (u32, Vec<f64>) {
    let improves = Rc::new(Cell::new(0));
    let counted = counting_shared_sqrt_stream(x, &improves);
    checked_prefix(&counted, x, elements, &improves)
}

/// Louis's version's prefix of `elements` elements for `x`, metered.
#[must_use]
fn louis_prefix(x: f64, elements: usize) -> (u32, Vec<f64>) {
    let improves = Rc::new(Cell::new(0));
    let counted = counting_louis_sqrt_stream(x, &improves);
    checked_prefix(&counted, x, elements, &improves)
}

mod ex_3_63 {
    use super::{louis_prefix, shared_prefix};

    /// Exercise 3.63: sqrt-stream memoization locality
    ///
    /// Answers how many `sqrt-improve` calls a walk of the first five
    /// and of the first six elements costs, in the shared
    /// self-referential `sqrt-stream` and in Louis's fresh-source
    /// version: `(shared_5, louis_5, shared_6, louis_6)`.
    #[must_use]
    pub fn ex_3_63() -> (u32, u32, u32, u32) {
        // Five elements, indices 0 through 4; then six, indices 0
        // through 5. Each prefix runs on a fresh stream with a fresh
        // meter, so the count is what one walk costs.
        let (shared_5, _) = shared_prefix(2.0, 5);
        let (louis_5, _) = louis_prefix(2.0, 5);
        let (shared_6, _) = shared_prefix(2.0, 6);
        let (louis_6, _) = louis_prefix(2.0, 6);
        (shared_5, louis_5, shared_6, louis_6)
    }
}

#[test]
fn ex_3_63() {
    let (shared_5, louis_5, shared_6, louis_6) = ex_3_63::ex_3_63();
    // One shared memoized spine: the walk of k elements forces one
    // more tail per element it advances past, and each forced tail's
    // map head is exactly one improve of the previous guess --
    // k improve calls for a k-element walk, measured.
    assert_eq!(shared_5, 5);
    assert_eq!(shared_6, 6);
    // Louis's tower: element 1 costs the level-1 map head, element 2
    // re-derives the map heads of the two fresh levels below it, and
    // so on -- a k-element walk costs 1 + 2 + ... + k = k(k+1)/2,
    // the redundant computation Alyssa names, measured.
    assert_eq!(louis_5, 15);
    assert_eq!(louis_6, 21);
    // Both metered twins were checked element-for-element against the
    // module's `sqrt_stream` inside `checked_prefix` on every run.
}
