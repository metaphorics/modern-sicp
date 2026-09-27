// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 1.1, listing 6: two squares, block structure, and lexical scoping.

use ch01::sec_1_1::{sqrt_block, sqrt_lexical, square};

fn main() {
    {
        fn double(x: f64) -> f64 {
            x + x
        }

        fn square_by_log(x: f64) -> f64 {
            double(x.ln()).exp()
        }

        println!("{}", square_by_log(4.0));
        // => 15.999999999999998
        assert!((square_by_log(4.0) - 16.0).abs() < 1e-12);
    }

    let root = sqrt_block(16.0);
    println!("{root}");
    // => 4.000000636692939
    assert!((root - 4.0).abs() < 0.001);

    let root = sqrt_lexical(16.0);
    println!("{root}");
    // => 4.000000636692939
    assert!((root - 4.0).abs() < 0.001);

    println!("{}", square(5.0));
    // => 25
    assert!((square(5.0) - 25.0_f64).abs() < 1e-9);
}
