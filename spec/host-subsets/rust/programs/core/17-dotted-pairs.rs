// SPDX-License-Identifier: GPL-3.0-only
// Case: core/17-dotted-pairs. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Dotted pairs as owned product data: the cons-cell lesson with the
/// mutation the section's set-car exercises need.
struct Pair {
    car: i64,
    cdr: i64,
}

fn cons(car: i64, cdr: i64) -> Pair {
    Pair { car, cdr }
}

fn main() {
    let mut pair = cons(1, 2);
    println!("{}", pair.car);
    println!("{}", pair.cdr);
    pair.car = 10;
    pair.cdr = 20;
    println!("{}", pair.car + pair.cdr);
}
