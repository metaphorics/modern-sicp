// SPDX-License-Identifier: GPL-3.0-only
// Case: core/14-higher-order. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// First-class procedures: function pointers are values, combinable
/// through admitted function types.
fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn twice(f: fn(i64, i64) -> i64, x: i64) -> i64 {
    f(x, x)
}

fn main() {
    println!("{}", twice(add, 7));
    println!("{}", twice(mul, 7));
    let op: fn(i64, i64) -> i64 = add;
    println!("{}", op(20, 22));
}
