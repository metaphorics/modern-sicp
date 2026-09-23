// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.6: Closures and trait objects.

/// Returns a closure that multiplies its argument by `factor`; the
/// closure captures `factor` by value (`move`), so it outlives this call.
pub fn make_multiplier(factor: i64) -> impl Fn(i64) -> i64 {
    move |x| x * factor
}

/// Applies each of `ops` to `x` in order, collecting the results. `dyn
/// Fn` lets one slice hold closures of different concrete types, the way
/// the book's tables hold procedures of different shapes.
#[must_use]
pub fn apply_all(ops: &[Box<dyn Fn(i64) -> i64>], x: i64) -> Vec<i64> {
    ops.iter().map(|op| op(x)).collect()
}
