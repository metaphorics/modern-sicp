// SPDX-License-Identifier: GPL-3.0-only
// Case: compiler/02-factorial. Observation class: value.
// Provenance: derived; parent confirms natively.
//
// The lesson: compiled recursive calls agree with the direct engine.

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(6));
}
