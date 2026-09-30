// SPDX-License-Identifier: GPL-3.0-only
// Case: core/05-gcd. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Euclid's algorithm as an iterative process.
fn gcd(a: i64, b: i64) -> i64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

fn main() {
    println!("{}", gcd(206, 40));
    println!("{}", gcd(17, 5));
}
