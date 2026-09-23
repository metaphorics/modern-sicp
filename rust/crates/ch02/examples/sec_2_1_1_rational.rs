// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.1.1

//! Section 2.1.1: pairs, and the rational-number package built on the
//! ideas they demonstrate.

use ch02::sec_2_1::{Rational, car, cdr, cons};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pairs: a compound data object built with `cons`, taken apart with
    // `car` and `cdr`.
    let x = cons(1, 2);
    let first_of_x = car(&x);
    let second_of_x = cdr(&x);
    println!("{first_of_x}");
    // => 1
    println!("{second_of_x}");
    // => 2
    assert_eq!((first_of_x, second_of_x), (1, 2));

    // A pair can hold pairs: this is the only ability compound data
    // needs to build arbitrarily complex structures (section 2.2).
    let y = cons(3, 4);
    let z = cons(x, y);
    let nested_first = car(&car(&z));
    let nested_second = car(&cdr(&z));
    println!("{nested_first}");
    // => 1
    println!("{nested_second}");
    // => 3
    assert_eq!((nested_first, nested_second), (1, 3));

    // Rational-number arithmetic, wishful-thinking style: `add`, `sub`,
    // `mul`, and `div` are written in terms of `numer`, `denom`, and
    // `new`, before this file shows how those three are implemented.
    let one_half = Rational::new(1, 2)?;
    println!("{one_half}");
    // => 1/2
    assert_eq!(one_half.to_string(), "1/2");

    let one_third = Rational::new(1, 3)?;
    let sum = one_half.add(&one_third)?;
    let product = one_half.mul(&one_third)?;
    println!("{sum}");
    // => 5/6
    println!("{product}");
    // => 1/6
    assert_eq!(sum.to_string(), "5/6");
    assert_eq!(product.to_string(), "1/6");

    // Without reduction, adding a third to itself would print 6/9: the
    // numerator and denominator computed directly by `add-rat`'s formula,
    // `(n1*d2 + n2*d1) / (d1*d2)` with `n1 = n2 = 1` and `d1 = d2 = 3`,
    // with no `gcd` step.
    let (third_numer, third_denom) = (1_i128, 3_i128);
    let unreduced_num = third_numer * third_denom + third_numer * third_denom;
    let unreduced_den = third_denom * third_denom;
    println!("{unreduced_num}/{unreduced_den}");
    // => 6/9
    assert_eq!((unreduced_num, unreduced_den), (6, 9));

    // `Rational::new` reduces to lowest terms by construction, so the
    // same sum through the real package prints 2/3, as desired.
    let reduced_sum = one_third.add(&one_third)?;
    println!("{reduced_sum}");
    // => 2/3
    assert_eq!(reduced_sum.to_string(), "2/3");

    Ok(())
}
