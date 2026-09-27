// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.5.5

//! Section 3.5.5: Modularity of functional programs and modularity of
//! objects -- the random-number stream with no hidden state, the Cesaro
//! experiment on consecutive pairs, Monte Carlo estimation of pi as one
//! stream of estimates, and the withdrawal processor as a mathematical
//! function over a stream of amounts.

use ch03::sec_3_1::RANDOM_INIT;
use ch03::sec_3_5::{
    integers, pi_estimates, rand_update, random_numbers, stream_map, stream_ref, stream_withdraw,
};

fn main() {
    // The stream of random words starts from the edition's fixed seed
    // and extends by mapping rand-update over itself: random-init, then
    // the successive rand-update values, each a pure function of the
    // word before it.
    let first_word = stream_ref(&random_numbers(), 0);
    println!("{first_word}");
    // => 1
    assert_eq!(first_word, RANDOM_INIT);
    let second_word = stream_ref(&random_numbers(), 1);
    println!("{second_word}");
    // => 5180492295206395165
    assert_eq!(second_word, rand_update(RANDOM_INIT));
    assert_eq!(second_word, 5_180_492_295_206_395_165);
    let third_word = stream_ref(&random_numbers(), 2);
    println!("{third_word}");
    // => 2586950713725923525
    assert_eq!(third_word, rand_update(second_word));
    assert_eq!(third_word, 2_586_950_713_725_923_525);

    // Each estimate of pi from a Cesaro run: the estimates sharpen as
    // the consumer looks farther down the same stream.
    let estimates = pi_estimates();
    for index in [100usize, 1000, 10_000] {
        let estimate = stream_ref(&estimates, index);
        println!("{estimate}");
        // => one value in each case, within 0.5, 0.2, and 0.05 of pi
        let tolerance = match index {
            100 => 0.5,
            1000 => 0.2,
            _ => 0.05,
        };
        assert!(
            (estimate - std::f64::consts::PI).abs() < tolerance,
            "estimate {estimate} at {index} trials left the tolerance"
        );
    }

    // The withdrawal processor of the section: a function from a balance
    // and a stream of amounts to the stream of balances, with no state
    // anywhere, showing the same behavior as the object of 3.1.3.
    let amounts = stream_map(|_| 25, &integers());
    let balances = stream_withdraw(100, &amounts);
    let prefix: Vec<i128> = balances.iter().take(4).collect();
    println!("{prefix:?}");
    // => [100, 75, 50, 25]
    assert_eq!(prefix, [100, 75, 50, 25]);
    println!("{}", stream_ref(&balances, 10));
    // => -150
    assert_eq!(stream_ref(&balances, 10), -150);
}
