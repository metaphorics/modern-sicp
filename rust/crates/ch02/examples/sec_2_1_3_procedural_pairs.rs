// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.1.3

//! Section 2.1.3: what is meant by data, demonstrated by pairs built
//! entirely out of procedures (no data structure at all) and by Church
//! numerals, which represent nonnegative integers the same way.

use ch02::sec_2_1::{car_proc, cdr_proc, church_succ, church_zero, cons_proc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A pair with no data structure behind it: `cons_proc` returns a
    // dispatch closure, and `car_proc`/`cdr_proc` are the only two
    // messages it answers.
    let pair = cons_proc(1, 2);
    let first_of_pair = car_proc(&pair)?;
    let second_of_pair = cdr_proc(&pair)?;
    println!("{first_of_pair}");
    // => 1
    println!("{second_of_pair}");
    // => 2
    assert_eq!((first_of_pair, second_of_pair), (1, 2));

    // Church numerals: `zero` applies its argument function zero times;
    // `succ` applies it one more time than its argument does. Converting
    // back to an ordinary integer, by counting the applications, is how
    // this file checks the numerals.
    let zero = church_zero();
    let one = church_succ(&zero);
    let two = church_succ(&one);
    let three = church_succ(&two);
    println!("{}", zero.to_i128());
    // => 0
    println!("{}", three.to_i128());
    // => 3
    assert_eq!(zero.to_i128(), 0);
    assert_eq!(one.to_i128(), 1);
    assert_eq!(two.to_i128(), 2);
    assert_eq!(three.to_i128(), 3);

    Ok(())
}
