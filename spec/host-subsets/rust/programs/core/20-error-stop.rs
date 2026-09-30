// SPDX-License-Identifier: GPL-3.0-only
// Case: core/20-error-stop. Observation class: error. Provenance:
// derived from teaching execution; parent confirms natively.
//
// The lesson: a checked-build trap stops the run. The program prints
// one line and then divides by zero, which grammar §6.4 classifies as
// a runtime trap, not an ordinary error value. The division goes
// through an ordinary function parameter so it is a runtime division.

fn divide(numerator: i64, denominator: i64) -> i64 {
    numerator / denominator
}

fn main() {
    println!("before");
    let bad = divide(6, 0);
    println!("{}", bad);
}
