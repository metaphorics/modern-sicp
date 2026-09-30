// SPDX-License-Identifier: GPL-3.0-only
// Case: core/07-fibonacci. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Tree recursion: two recursive calls per activation.
fn fib(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!("{}", fib(10));
    println!("{}", fib(15));
}
