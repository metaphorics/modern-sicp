// SPDX-License-Identifier: GPL-3.0-only
// Case: core/19-deep-recursion. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Deep non-tail recursion: the activation chain is the lesson, and
/// the depth stays inside every engine's stack discipline.
fn sum_to(n: i64) -> i64 {
    if n == 0 {
        0
    } else {
        n + sum_to(n - 1)
    }
}

fn main() {
    println!("{}", sum_to(10000));
    println!("{}", sum_to(100));
}
