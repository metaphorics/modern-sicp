// SPDX-License-Identifier: GPL-3.0-only
// Case: core/04-sqrt-newton (spec/host-subsets/cases.json).
// Provenance: derived from teaching-engine execution; the parent's
// native run confirms the expectation after entrypoints freeze. This
// file is never labeled native-proven.
//
// The lesson: Newton's method for square roots. The accepted Rust
// subset is i64-only (no floating point), so the iteration runs on a
// justified fixed-point representation: every value is the true value
// scaled by 2^20, written as the literal 1048576. The update
//
//     guess' = (guess + x / guess) / 2
//
// is `improve` below; the program prints the fixed-point root of 2 and
// a whole-number root. This is the lesson's iteration translated
// explicitly into the contract's numeric rules (i64 arithmetic only).

/// One Newton step on the scaled representation: the scale is
/// 1048576 (2^20), folded into the literals below.
fn improve(x_fixed: i64, guess: i64) -> i64 {
    let scale = 1048576;
    let quotient = (x_fixed * scale) / guess;
    (guess + quotient) / 2
}

/// Newton's iteration from a fixed first guess of one (scaled).
fn sqrt_fixed(x_fixed: i64) -> i64 {
    let scale = 1048576;
    let mut guess = scale;
    let mut count: i64 = 0;
    while count < 24 {
        guess = improve(x_fixed, guess);
        count += 1;
    }
    guess
}

/// Prints the fixed-point root of 2 and of a perfect square.
fn main() {
    let scale = 1048576;
    let two = 2 * scale;
    let root_two = sqrt_fixed(two);
    println!("{}", root_two);
    let hundred = 100 * scale;
    let root_hundred = sqrt_fixed(hundred);
    println!("{}", root_hundred);
}
