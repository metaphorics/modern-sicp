// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.2

//! Section 1.2.6: testing for primality, trial division and the Fermat
//! test over the runtime's seeded generator.

use ch01::sec_1_2::{fast_prime, is_prime, smallest_divisor};
use sicp_runtime::Random;

fn main() {
    let divisor_of_199 = smallest_divisor(199);
    println!("{divisor_of_199}");
    // => 199
    assert_eq!(divisor_of_199, 199);

    let divisor_of_19999 = smallest_divisor(19999);
    println!("{divisor_of_19999}");
    // => 7
    assert_eq!(divisor_of_19999, 7);

    let one_hundred_one = is_prime(101);
    println!("{one_hundred_one}");
    // => true
    assert!(one_hundred_one);

    let mut rng = Random::new(1).expect("seed 1 is valid");
    let thousand_nine_fast = fast_prime(1009, 3, &mut rng);
    println!("{thousand_nine_fast}");
    // => true
    assert!(thousand_nine_fast);

    let around_a_million = fast_prime(1_000_003, 3, &mut rng);
    println!("{around_a_million}");
    // => true
    assert!(around_a_million);
}
