// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.57: how many additions the
//! self-referential `fibs` performs. A counting adder replaces
//! `add-streams` and increments a meter once per stream element it
//! builds; because every tail thunk is memoized, each element is built
//! once, so producing element `n` costs exactly `n - 1` additions (the
//! 0 and 1 seeds need none) and a second walk costs none. The contrast:
//! the same computation driven by plain tree recursion, which is what
//! re-forcing unmemoized delays amounts to, makes a number of calls
//! that grows like `fib(n)` itself.

use std::cell::Cell;
use std::rc::Rc;

use ch03::sec_3_5::{Stream, cons_stream, self_stream, stream_ref};

/// The book's `add-streams` with a meter: the elementwise sum, counting
/// one addition per built cell. The delay structure is `stream_map2`'s,
/// so the meter sees exactly the element additions the definition of
/// `fibs` performs and nothing else.
fn counting_add_streams(
    s1: &Stream<i128>,
    s2: &Stream<i128>,
    additions: &Rc<Cell<u32>>,
) -> Stream<i128> {
    if s1.is_empty() || s2.is_empty() {
        return Stream::Empty;
    }
    let head = *s1.head() + *s2.head();
    additions.set(additions.get() + 1);
    let first = s1.clone();
    let second = s2.clone();
    let count = Rc::clone(additions);
    cons_stream(head, move || {
        counting_add_streams(&first.tail(), &second.tail(), &count)
    })
}

/// The book's `fibs` with the counting adder in place of
/// `add-streams`: 0 consed onto 1 consed onto the elementwise sum of
/// the stream's own tail and the stream itself. The one shared
/// `Rc<Cell<u32>>` rides along every clone, so every cell the spine
/// builds meters into the caller's cell.
fn counting_fibs(additions: &Rc<Cell<u32>>) -> Stream<i128> {
    let additions = Rc::clone(additions);
    self_stream(|fibs| {
        cons_stream(0, move || {
            cons_stream(1, {
                let named = fibs.clone();
                let count = Rc::clone(&additions);
                move || counting_add_streams(&named.stream().tail(), &named.stream(), &count)
            })
        })
    })
}

/// The tree-recursive `fib` of 1.2.2 with a call meter: one increment
/// per call. Re-forcing unmemoized delayed tails drives this same
/// recursion shape over stream elements, which is the exponential
/// growth the statement asks to demonstrate.
fn counted_tree_fib(n: u32, calls: &Cell<u32>) -> i128 {
    calls.set(calls.get() + 1);
    if n < 2 {
        i128::from(n)
    } else {
        counted_tree_fib(n - 1, calls) + counted_tree_fib(n - 2, calls)
    }
}

/// The measured answer: stream-element additions on the memoized spine,
/// plus the value and call count of the tree-recursive contrast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FibAdditions {
    /// The element produced, `fibs[10]`.
    pub element_10: i128,
    /// Additions consumed producing `element_10`.
    pub additions_10: u32,
    /// The element produced, `fibs[20]`.
    pub element_20: i128,
    /// Additions consumed producing `element_20`.
    pub additions_20: u32,
    /// Additions a second identical walk performs: the memoized spine
    /// is shared, so re-reading adds nothing.
    pub additions_reread: u32,
    /// `fib(15)` by plain tree recursion.
    pub tree_fib_15: i128,
    /// Calls the tree recursion made for that value.
    pub tree_calls_15: u32,
}

mod ex_3_57 {
    use super::{Cell, FibAdditions, Rc, counted_tree_fib, counting_fibs, stream_ref};

    /// Exercise 3.57: fib additions with memoized delay
    ///
    /// Meters the additions of producing the 10th and 20th elements of
    /// the counting `fibs` and a second identical walk, then meters the
    /// tree-recursive `fib(15)` the unmemoized delay would amount to.
    #[must_use]
    pub fn ex_3_57() -> FibAdditions {
        let counter10 = Rc::new(Cell::new(0));
        let element_10 = stream_ref(&counting_fibs(&counter10), 10);

        let counter20 = Rc::new(Cell::new(0));
        let fibs20 = counting_fibs(&counter20);
        let element_20 = stream_ref(&fibs20, 20);

        let additions_before_reread = counter20.get();
        let reread_element_20 = stream_ref(&fibs20, 20);
        let additions_reread = counter20.get() - additions_before_reread;
        // The second walk answers the same element; only its meter
        // delta, zero, is the point.
        let _ = reread_element_20;

        let calls = Cell::new(0);
        let tree_fib_15 = counted_tree_fib(15, &calls);
        FibAdditions {
            element_10,
            additions_10: counter10.get(),
            element_20,
            additions_20: counter20.get() - additions_reread,
            additions_reread,
            tree_fib_15,
            tree_calls_15: calls.get(),
        }
    }
}

#[test]
fn ex_3_57() {
    let report = ex_3_57::ex_3_57();
    // Producing element n builds one new add-streams cell per element
    // from index 2 up to n -- the seeds 0 and 1 need no addition -- and
    // memoization builds each cell exactly once: n - 1 additions. The
    // elements themselves are Fibonacci's: 55 and 6765.
    assert_eq!(report.element_10, 55);
    assert_eq!(report.additions_10, 9);
    assert_eq!(report.element_20, 6765);
    assert_eq!(report.additions_20, 19);
    // The second walk re-reads the memoized spine: zero additions.
    assert_eq!(report.additions_reread, 0);
    // The tree-recursive contrast: fib(15) = 610 either way, but the
    // unmemoized form makes 1973 calls = 2 x fib(16) - 1, growing like
    // fib(n) itself instead of linearly in n.
    assert_eq!(report.tree_fib_15, 610);
    assert_eq!(report.tree_calls_15, 1973);
}
