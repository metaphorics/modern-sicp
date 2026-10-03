// SPDX-License-Identifier: GPL-3.0-only
// Case: core/06-expt. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Linear recursion: one recursive call per activation.
fn expt(base: i64, power: i64) -> i64 {
    if power == 0 {
        1
    } else {
        base * expt(base, power - 1)
    }
}

fn main() {
    println!("{}", expt(2, 10));
    println!("{}", expt(3, 4));
}
