// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.1: Values, numbers, and bindings.

/// Squares `n` under checked multiplication, returning [`None`] where the
/// exact result would overflow `i128` instead of wrapping to a silently
/// wrong value.
#[must_use]
pub fn checked_square(n: i128) -> Option<i128> {
    n.checked_mul(n)
}
