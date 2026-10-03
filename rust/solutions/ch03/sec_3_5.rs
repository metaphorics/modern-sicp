// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercise 3.5 and its tailored addition
//! 3.5a: Monte Carlo integration, and the seeded stream plus Cesaro
//! table.

use ch03::sec_3_1::RANDOM_INIT;

mod ex_3_05 {
    use ch03::sec_3_1::{RANDOM_INIT, Rand, monte_carlo};

    /// The book's `random-in-range`: a float drawn uniformly from
    /// `low` (inclusive) to `high` (exclusive), scaled from the
    /// generator's 64-bit word.
    //
    // The single cast from 128-bit integer to 64-bit float is this
    // edition's accepted spelling for scaling a generator word into a
    // unit interval; the estimate stays far inside its tolerance.
    #[allow(clippy::cast_precision_loss)]
    pub fn random_in_range(rand: &mut Rand, low: f64, high: f64) -> f64 {
        let range = high - low;
        low + i128::from(rand.generate()) as f64 / i128::from(u64::MAX) as f64 * range
    }

    /// Exercise 3.5: Monte Carlo integration
    ///
    /// Estimates the area of the region picked out by `p` inside the
    /// rectangle `[x1, x2] x [y1, y2]`: the fraction of random points
    /// satisfying the predicate, times the rectangle's area. The
    /// generator is captured once, in the experiment closure, exactly
    /// where the book's `rand` hides.
    pub fn estimate_integral(
        p: &mut dyn FnMut(f64, f64) -> bool,
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        trials: u32,
        rand: &mut Rand,
    ) -> f64 {
        let rectangle_area = (x2 - x1) * (y2 - y1);
        let mut experiment = || {
            let x = random_in_range(rand, x1, x2);
            let y = random_in_range(rand, y1, y2);
            p(x, y)
        };
        monte_carlo(trials, &mut experiment) * rectangle_area
    }

    /// Exercise 3.5: Monte Carlo integration over the unit circle
    ///
    /// Returns the estimate of pi from 10,000 random points in the
    /// square from (0, 0) to (2, 2), measured by the area of the circle
    /// of radius 1 centered at (1, 1).
    #[must_use]
    pub fn ex_3_05() -> f64 {
        let mut rand = Rand::new(RANDOM_INIT).expect("RANDOM_INIT is the nonzero constant 1");
        let mut in_unit_circle = |x: f64, y: f64| (x - 1.0).powi(2) + (y - 1.0).powi(2) <= 1.0;
        estimate_integral(&mut in_unit_circle, 0.0, 2.0, 0.0, 2.0, 10_000, &mut rand)
    }
}

#[test]
fn ex_3_05() {
    let estimate = ex_3_05::ex_3_05();
    assert!((estimate - std::f64::consts::PI).abs() < 0.25);
    assert!((estimate - 3.1228).abs() < 1e-3);
}

mod ex_3_05a {
    use ch03::sec_3_1::{RANDOM_INIT, Rand, estimate_pi};
    use sicp_runtime::SicpError;

    /// Exercise 3.5a (this edition): a seeded stream and a Cesaro table
    ///
    /// Draws the first `count` numbers of the generator started from
    /// `seed`. The stream is a pure function of the seed, which is what
    /// makes every probabilistic interaction of this edition
    /// reproducible.
    ///
    /// # Errors
    /// Propagates the generator's zero-seed rejection; `xorshift64*`
    /// maps zero to zero forever.
    pub fn seeded_stream(seed: u64, count: usize) -> Result<Vec<u64>, SicpError> {
        let mut rand = Rand::new(seed)?;
        Ok((0..count).map(|_| rand.generate()).collect())
    }

    /// Exercise 3.5a (this edition): the Cesaro table
    ///
    /// Estimates pi at 100, 1,000, and 10,000 trials, each entry from a
    /// generator freshly seeded at the section's fixed start.
    #[must_use]
    pub fn cesaro_estimates() -> [f64; 3] {
        [
            estimate_pi(100, &mut fresh_rand()),
            estimate_pi(1_000, &mut fresh_rand()),
            estimate_pi(10_000, &mut fresh_rand()),
        ]
    }

    fn fresh_rand() -> Rand {
        Rand::new(RANDOM_INIT).expect("RANDOM_INIT is the nonzero constant 1")
    }

    /// Exercise 3.5a: the seeded stream snapshot and the Cesaro table
    ///
    /// Returns the first five numbers of the generator started from the
    /// section's fixed seed, and the estimates at 100, 1,000, and
    /// 10,000 trials.
    #[must_use]
    pub fn ex_3_05a() -> ([u64; 5], [f64; 3]) {
        let stream = seeded_stream(RANDOM_INIT, 5).expect("RANDOM_INIT is valid");
        let mut five = [0_u64; 5];
        five.copy_from_slice(&stream);
        (five, cesaro_estimates())
    }
}

#[test]
fn ex_3_05a() {
    // The stream for a fixed seed is a constant: two independent
    // generators replay it identically, draw for draw.
    let (stream, estimates) = ex_3_05a::ex_3_05a();
    assert_eq!(
        stream,
        [
            5_180_492_295_206_395_165,
            2_586_950_713_725_923_525,
            3_968_523_955_086_520_050,
            17_304_925_549_865_963_664,
            4_623_165_523_142_949_034,
        ]
    );
    let replay = ex_3_05a::seeded_stream(RANDOM_INIT, 5).expect("RANDOM_INIT is valid");
    assert_eq!(stream.as_slice(), replay.as_slice());

    // The Cesaro estimates close in on pi as the trial count grows, and
    // repeat exactly on a re-run.
    let pi = std::f64::consts::PI;
    for (estimate, tolerance) in estimates.iter().zip([0.5, 0.2, 0.05]) {
        assert!((estimate - pi).abs() < tolerance);
    }
    let rerun = ex_3_05a::cesaro_estimates();
    for (first, second) in estimates.iter().zip(rerun) {
        assert!((first - second).abs() < 1e-12);
    }
}
