// SPDX-License-Identifier: GPL-3.0-only
// Case: compiler/01-primitive-combination. Observation class: value.
// Provenance: derived; parent confirms natively.
//
// The lesson: the compiler preserves ordinary primitive combination
// semantics across all engines.

fn combine(a: i64, b: i64) -> i64 {
    (a + b) * (a - b) + a * b
}

fn main() {
    println!("{}", combine(7, 3));
}
