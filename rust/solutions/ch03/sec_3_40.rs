// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.40: every interleaving of the
//! squaring and cubing processes, simulated over their read and write
//! steps, leaves exactly {100, 1000, 10000, 100000, 1000000}; serialized,
//! both orders land on 1000000.

use std::sync::{Arc, Mutex};

use ch03::sec_3_4::{Serializer, SharedInt, cube_x, parallel_execute, read_shared, square_x};

/// One step of a simulated process: a read multiplies the current value
/// of `x` into the process's product, the write stores the product back.
/// `x * x` is two reads and a write; `x * x * x` is three.
#[derive(Clone, Copy)]
enum Step {
    /// Multiply the current `x` into the product.
    Read,
    /// Store the product into `x`.
    Write,
}

/// The squaring process: P1 of the exercise.
const P1_STEPS: [Step; 3] = [Step::Read, Step::Read, Step::Write];

/// The cubing process: P2 of the exercise.
const P2_STEPS: [Step; 4] = [Step::Read, Step::Read, Step::Read, Step::Write];

/// Simulates one interleaving. The schedule has seven entries, one per
/// step; `First` means P1 takes its next step, `Second` means P2 does,
/// and each process's own steps keep their order.
#[must_use]
fn simulate(schedule: &[Turn]) -> i128 {
    let mut x = 10_i128;
    let mut p1 = 1_i128;
    let mut p2 = 1_i128;
    let mut i1 = 0_usize;
    let mut i2 = 0_usize;
    for turn in schedule {
        match *turn {
            Turn::First => {
                x = take_step(x, &mut p1, &mut i1, &P1_STEPS);
            }
            Turn::Second => {
                x = take_step(x, &mut p2, &mut i2, &P2_STEPS);
            }
        }
    }
    x
}

/// Runs one step of one simulated process and answers the new `x`.
fn take_step(x: i128, product: &mut i128, index: &mut usize, steps: &[Step]) -> i128 {
    let step = steps[*index];
    *index += 1;
    match step {
        Step::Read => {
            *product *= x;
            x
        }
        Step::Write => *product,
    }
}

/// Whose turn a schedule slot gives a step to.
#[derive(Clone, Copy)]
enum Turn {
    /// The squaring process.
    First,
    /// The cubing process.
    Second,
}

/// Enumerates every interleaving that respects each process's own order
/// -- all ways to weave three P1 steps and four P2 steps -- and answers
/// the sorted set of final values.
#[must_use]
fn all_outcomes() -> Vec<i128> {
    let mut seen = std::collections::BTreeSet::new();
    for mask in 0..(1_u32 << 7) {
        let schedule: Vec<Turn> = (0..7)
            .map(|bit| {
                if (mask >> bit) & 1 == 1 {
                    Turn::First
                } else {
                    Turn::Second
                }
            })
            .collect();
        let firsts = (0..7).filter(|bit| (mask >> bit) & 1 == 1).count();
        if firsts == 3 {
            seen.insert(simulate(&schedule));
        }
    }
    seen.into_iter().collect()
}

/// Runs the serialized race once for real and answers the value left.
#[must_use]
fn serialized_run() -> i128 {
    let x: SharedInt = Arc::new(Mutex::new(10));
    let x2 = Arc::clone(&x);
    let s = Serializer::new();
    let s2 = s.clone();
    let _ = parallel_execute(
        |_run| s.protect(|| square_x(&x)),
        |_run| s2.protect(|| cube_x(&x2)),
    );
    read_shared(&x)
}

mod ex_3_40 {
    use super::{all_outcomes, serialized_run};

    /// Exercise 3.40: all values of concurrent x squared
    ///
    /// Answers the sorted values the unserialized interleavings can
    /// leave, and the value a serialized run leaves.
    #[must_use]
    pub fn ex_3_40() -> (Vec<i128>, i128) {
        (all_outcomes(), serialized_run())
    }
}

#[test]
fn ex_3_40() {
    let (outcomes, serialized) = ex_3_40::ex_3_40();
    // Squaring writes 10·10 = 100 or a product that has read the cube's
    // 1000; cubing writes 10³ = 1000 or a product that has read the
    // square's 100. Five values in all.
    assert_eq!(outcomes, vec![100, 1000, 10_000, 100_000, 1_000_000]);
    // Serialized, whoever goes second reads the other's result, and both
    // orders compute (10²)³ = (10³)² = one million.
    assert_eq!(serialized, 1_000_000);
    // The unserialized set contains the serialized value: serialization
    // narrows the outcomes, it does not invent new ones.
    assert!(outcomes.contains(&serialized));
}
