// SPDX-License-Identifier: GPL-3.0-only
// Case: core/01-square. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// The book's first procedure: `square`.
fn square(x: i64) -> i64 {
    x * x
}

fn main() {
    println!("{}", square(5));
    println!("{}", square(-3));
}
