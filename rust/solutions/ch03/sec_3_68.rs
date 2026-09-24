// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.68: Louis Reasoner's pairs,
//! which works with the whole first row and appends it to the recursive
//! call. Translated, `stream-append` takes the recursive `pairs` call
//! as an eager second argument, so building the stream past the row's
//! head demands a construction that never terminates. The run shows the
//! two halves of that failure with an `Rc<Cell<u32>>` meter in the
//! row-mapping closure and a step budget inside
//! `std::panic::catch_unwind`: the row's eager head -- the value at
//! index 0 -- computes in one metered step, while the tail-building
//! needed for index 1 and beyond burns the full budget and produces
//! nothing.

use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use ch03::sec_3_5::{Stream, cons_stream, integers, stream_map};

/// The step budget of one tail-building attempt: far past any amount of
/// honest work the construction could do, yet small enough that the
/// doomed recursion panics well inside a test thread's stack.
const SECOND_ELEMENT_BUDGET: u32 = 4_000;

/// The book's `stream-append`: the elements of `s1` followed by the
/// elements of `s2`, empties passing through.
fn stream_append<A: Clone + 'static>(s1: &Stream<A>, s2: &Stream<A>) -> Stream<A> {
    if s1.is_empty() {
        return s2.clone();
    }
    let head = s1.head().clone();
    let front = s1.clone();
    let back = s2.clone();
    cons_stream(head, move || stream_append(&front.tail(), &back))
}

/// The mapped whole first row of Louis's pairs, with the meter in the
/// per-element closure. The head is eager, so building a row level
/// ticks the meter exactly once; at the budget the meter panics, which
/// is what turns Louis's unbounded recursion into a measurement.
fn mapped_first_row(
    s: &Stream<i128>,
    t: &Stream<i128>,
    steps: &Rc<Cell<u32>>,
    budget: u32,
) -> Stream<(i128, i128)> {
    let meter = Rc::clone(steps);
    let first = *s.head();
    stream_map(
        move |x: &i128| {
            meter.set(meter.get() + 1);
            assert!(
                meter.get() < budget,
                "Louis pairs construction burned its {budget}-step budget"
            );
            (first, *x)
        },
        t,
    )
}

/// Louis's `pairs`: the whole first row, stream-appended to the
/// recursive call as `stream_append`'s eager second argument --
/// `(stream-append (stream-map (lambda (x) (list (stream-car s) x)) t)
/// (pairs (stream-cdr s) (stream-cdr t)))`. Building the stream runs
/// the recursion before `stream_append` can serve a single element, so
/// construction never returns; the meter's budget bounds the dive.
#[expect(
    unconditional_recursion,
    reason = "the unconditional recursion is Louis's defect under demonstration"
)]
fn pairs_louis(
    s: &Stream<i128>,
    t: &Stream<i128>,
    steps: &Rc<Cell<u32>>,
    budget: u32,
) -> Stream<(i128, i128)> {
    let row = mapped_first_row(s, t, steps, budget);
    stream_append(&row, &pairs_louis(&s.tail(), &t.tail(), steps, budget))
}

mod ex_3_68 {
    use super::{
        AssertUnwindSafe, Cell, Rc, SECOND_ELEMENT_BUDGET, integers, mapped_first_row, pairs_louis,
    };

    /// Exercise 3.68: Louis pairs infinite recursion
    ///
    /// Answers the value of the first element, whether the attempt to
    /// build past it died in the metered budget panic, and the meter's
    /// final count: `((1, 1), true, budget)` on the working run.
    ///
    /// # Panics
    /// Panics if the budgeted construction returns instead of panicking,
    /// which would mean Louis's recursion terminated.
    #[must_use]
    pub fn ex_3_68() -> ((i128, i128), bool, u32) {
        let index_zero_steps = Rc::new(Cell::new(0));
        let integers = integers();
        // Index 0 is fine: the row's eager head is the stream's first
        // element, one metered step, no recursion touched.
        let row = mapped_first_row(
            &integers,
            &integers,
            &index_zero_steps,
            SECOND_ELEMENT_BUDGET,
        );
        let first = *row.head();
        // From index 1 on, every element needs the appended recursive
        // stream, whose construction dives without end. The budget
        // turns the dive into a caught, counted panic.
        let tail_steps = Rc::new(Cell::new(0));
        let attempt = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let _ = pairs_louis(&integers, &integers, &tail_steps, SECOND_ELEMENT_BUDGET);
        }));
        let named_budget_panic = attempt.as_ref().err().is_some_and(|payload| {
            payload
                .downcast_ref::<String>()
                .is_some_and(|message| message.contains("budget"))
                || payload
                    .downcast_ref::<&str>()
                    .is_some_and(|message| message.contains("budget"))
        });
        assert!(
            named_budget_panic,
            "the tail-building must die in the budget panic"
        );
        assert!(
            attempt.is_err(),
            "Louis's recursive construction must not terminate"
        );
        (
            first,
            named_budget_panic && attempt.is_err(),
            tail_steps.get(),
        )
    }
}

#[test]
fn ex_3_68() {
    let (first, starved, tail_steps) = ex_3_68::ex_3_68();
    // The first element's value is the head pair of the row, computed
    // before any recursive construction: (1, 1).
    assert_eq!(first, (1, 1));
    // The tail-building for index 1 never finishes: it burned the full
    // budget, one meter tick per level of Louis's tower, and produced
    // no element. stream-append cannot incorporate the second stream
    // until the infinite first row is exhausted, and here the row's own
    // construction already demands the recursion as an eager argument.
    assert!(starved);
    assert_eq!(tail_steps, SECOND_ELEMENT_BUDGET);
}
