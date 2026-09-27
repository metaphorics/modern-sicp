// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.52: `accum` writes its running
//! total into one shared sum cell, so the cell reads off exactly how
//! much of `seq` the memoized delay has let run -- only the head at
//! `seq`'s definition, one filtered step per later definition, everything
//! through the requested index at `stream-ref`, and all twenty elements
//! after the full `z` walk. Rebuilding `z` over a fresh `seq` from the
//! same cell reproduces the statement's not-memoized counterfactual:
//! with no remembered elements, every access re-executes `accum` from
//! wherever the cell stands, and the same walk lands twice as high.

use std::cell::Cell;
use std::rc::Rc;

use ch03::sec_3_5::{Stream, stream_enumerate_interval, stream_filter, stream_map, stream_ref};

/// Builds the book's `seq` over `sum`: `accum` adds each element of
/// 1..=20 into the shared cell and answers the running total. The
/// mapping runs only as the stream is demanded -- one eager head, the
/// rest behind the memoized delay.
#[must_use]
fn accum_seq(sum: &Rc<Cell<i128>>) -> Stream<i128> {
    let cell = Rc::clone(sum);
    stream_map(
        move |x| {
            cell.set(cell.get() + *x);
            cell.get()
        },
        &stream_enumerate_interval(1, 20),
    )
}

/// Walks the book's expression sequence over one shared sum cell and
/// records the sum after each: `seq`, `y` (the evens), and `z` (the
/// multiples of 5) all share the one `seq`, so each definition forces
/// the filtered stream one step further. Answers the three definition
/// sums, `stream-ref y 7`, the full `z`, and the final sum.
#[must_use]
fn memoized_walk(sum: &Rc<Cell<i128>>) -> (Vec<i128>, i128, Vec<i128>, i128) {
    let seq = accum_seq(sum);
    let after_seq = sum.get();
    let y = stream_filter(|x| x % 2 == 0, &seq);
    let after_y = sum.get();
    let z = stream_filter(|x| x % 5 == 0, &seq);
    let after_z = sum.get();
    let build_sums = vec![after_seq, after_y, after_z];
    let at_seven = stream_ref(&y, 7);
    let z_values: Vec<i128> = z.iter().collect();
    (build_sums, at_seven, z_values, sum.get())
}

/// The statement's counterfactual: `z` rebuilt over a fresh `seq` from
/// the same sum cell, the shape the book's `delay` would take without
/// `memo-proc` -- nothing is remembered, so walking `z` re-executes
/// `accum` for the whole range. Answers the fresh `z` and the cell.
#[must_use]
fn unmemoized_rewalk(sum: &Rc<Cell<i128>>) -> (Vec<i128>, i128) {
    let seq = accum_seq(sum);
    let z = stream_filter(|x| x % 5 == 0, &seq);
    let z_values: Vec<i128> = z.iter().collect();
    (z_values, sum.get())
}

mod ex_3_52 {
    use std::cell::Cell;
    use std::rc::Rc;

    /// Exercise 3.52: accum traces assignment plus laziness
    ///
    /// Answers the memoized walk of the book's sequence -- the sum after
    /// each definition, `stream-ref y 7`, the full `z`, and the final
    /// sum -- then the counterfactual `z` and sum over a fresh `seq`
    /// from the same sum cell.
    #[must_use]
    pub fn ex_3_52() -> (Vec<i128>, i128, Vec<i128>, i128, Vec<i128>, i128) {
        let sum = Rc::new(Cell::new(0_i128));
        let (build_sums, at_seven, z_values, sum_final) = super::memoized_walk(&sum);
        let (fresh_z, fresh_sum) = super::unmemoized_rewalk(&sum);
        (
            build_sums, at_seven, z_values, sum_final, fresh_z, fresh_sum,
        )
    }
}

#[test]
fn ex_3_52() {
    let (build_sums, at_y_seven, z_values, sum_final, fresh_z, fresh_sum) = ex_3_52::ex_3_52();
    // The delay holds accum back: defining seq computes only its head
    // (accum 1 = 1), defining y forces seq to its first even element
    // (6), and defining z forces seq on to its first multiple of 5
    // (10).
    assert_eq!(build_sums, vec![1, 6, 10]);
    // The 8th even partial sum is 136, which is also the running sum:
    // reaching y index 7 computes seq through 1 + ... + 16 = 136.
    assert_eq!(at_y_seven, 136);
    // The full z walk scans seq to its end, so every multiple of 5 in
    // the 20-element seq comes out -- 10 15 45 55 105 120 and the two
    // trailing ones the scan uncovers -- and the running sum lands at
    // 210 = 1 + ... + 20.
    assert_eq!(z_values, vec![10, 15, 45, 55, 105, 120, 190, 210]);
    assert_eq!(sum_final, 210);
    // Not memoized: the fresh seq re-executes accum starting from the
    // cell's 210, so every element shifts up by exactly 210 and the
    // same walk lands at 420.
    assert_eq!(fresh_z, vec![220, 225, 255, 265, 315, 330, 400, 420]);
    assert_eq!(fresh_sum, 420);
}
