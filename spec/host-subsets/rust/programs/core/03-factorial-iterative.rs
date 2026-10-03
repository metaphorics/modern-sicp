// SPDX-License-Identifier: GPL-3.0-only
// Case: core/03-factorial-iterative. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// The book's iterative factorial: one loop, one accumulator.
fn factorial(n: i64) -> i64 {
    let mut product = 1;
    let mut counter = 1;
    while counter <= n {
        product = product * counter;
        counter += 1;
    }
    product
}

fn main() {
    println!("{}", factorial(5));
    println!("{}", factorial(10));
}
