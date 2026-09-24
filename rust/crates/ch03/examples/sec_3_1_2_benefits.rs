// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.1.2

//! Section 3.1.2: the benefits of introducing assignment — `rand` as an
//! object with hidden local state, and the Monte Carlo estimate of pi
//! with and without that state.

use ch03::sec_3_1::{RANDOM_INIT, Rand, estimate_pi, estimate_pi_stateless, rand_update};

fn main() {
    // `rand` carries its state in one hidden word: each call answers
    // with the next number of the stream. Seeded, the stream is fixed.
    let mut rand = Rand::new(RANDOM_INIT).expect("RANDOM_INIT is not zero");
    let draw = rand.generate();
    println!("{draw}");
    // => 5180492295206395165
    assert_eq!(draw, 5_180_492_295_206_395_165);

    let draw = rand.generate();
    println!("{draw}");
    // => 2586950713725923525
    assert_eq!(draw, 2_586_950_713_725_923_525);

    let draw = rand.generate();
    println!("{draw}");
    // => 3968523955086520050
    assert_eq!(draw, 3_968_523_955_086_520_050);

    // The Cesaro estimate of pi: six over the fraction of trials where
    // two random words are coprime, square-rooted. `rand` is captured
    // once at the edge, in `estimate_pi`.
    let pi = estimate_pi(10_000, &mut Rand::new(RANDOM_INIT).expect("seed is valid"));
    println!("{pi:.6}");
    // => 3.127036
    assert!((pi - std::f64::consts::PI).abs() < 0.05);

    // The same computation without local state: `rand_update` directly,
    // with the random words threaded through the loop by hand. The two
    // programs draw the very same numbers; only who remembers the state
    // differs.
    let threaded = estimate_pi_stateless(10_000, RANDOM_INIT);
    println!("{threaded:.6}");
    // => 3.127036
    assert!((threaded - pi).abs() < 1e-9);

    // `rand_update` itself is a mathematical function: the same word in,
    // the same word out, so the stream above replays identically from
    // the fixed seed.
    assert_eq!(rand_update(RANDOM_INIT), 5_180_492_295_206_395_165);
}
