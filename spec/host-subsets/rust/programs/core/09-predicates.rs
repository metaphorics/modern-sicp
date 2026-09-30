// SPDX-License-Identifier: GPL-3.0-only
// Case: core/09-predicates. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Predicates and the short-circuit discipline.
fn divides(a: i64, b: i64) -> bool {
    b % a == 0
}

fn even(n: i64) -> bool {
    divides(2, n)
}

fn in_range(n: i64, low: i64, high: i64) -> bool {
    low <= n && n <= high
}

fn main() {
    println!("{}", even(10));
    println!("{}", even(7));
    println!("{}", in_range(5, 1, 10));
    // The right operand of && never runs when the left is false: the
    // guard below divides by zero only if it is reached.
    let guarded = even(3) && divides(0, 1);
    println!("{}", guarded);
}
