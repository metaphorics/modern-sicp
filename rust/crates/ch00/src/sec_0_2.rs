// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.2: Functions, recursion, and iteration.

/// Sums `1..=n` by recursion: one stack frame per term, so a large enough
/// `n` overflows the call stack before `u64` ever would.
#[must_use]
pub fn sum_to_recursive(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n + sum_to_recursive(n - 1)
    }
}

/// Sums `1..=n` by iteration in one stack frame, however large `n` is.
#[must_use]
pub fn sum_to_iterative(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}
