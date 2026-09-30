// SPDX-License-Identifier: GPL-3.0-only
// Case: core/02-factorial-recursive. Provenance: derived from teaching
// execution; parent confirms natively. Never native-proven.

/// The book's recursive factorial.
fn factorial(n: i64) -> i64 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(5));
    println!("{}", factorial(10));
}
