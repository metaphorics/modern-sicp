// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";

import { makeRand, monteCarlo } from "../../packages/ch3/src/01-assignment.js";

/**
 * Exercise 3.5: Monte Carlo integration. The experiment draws a point
 * from the rectangle and runs the predicate; the same `monteCarlo` of
 * the section turns trials into the fraction inside the region; the
 * estimate multiplies the fraction by the rectangle's area. Randomness
 * is an `Effect<number>` argument (the section's seeded generator), so
 * the estimate is reproducible instead of hostage to `Math.random`.
 */

/** The book's `random-in-range`: a number uniform in [low, high), drawn
 * from the 32-bit generator by scaling into the range. */
export const randomInRange = (
  low: number,
  high: number,
  rand: Effect.Effect<number>,
): Effect.Effect<number> => Effect.map(rand, (r) => low + (high - low) * (r / 4294967296));

/** The book's `estimate-integral`: estimate the area of the region the
 * `predicate` describes inside the rectangle (x1, y1) to (x2, y2), by
 * `trials` random points. */
export const estimateIntegral = (
  predicate: (x: number, y: number) => boolean,
  x1: number,
  x2: number,
  y1: number,
  y2: number,
  trials: number,
  rand: Effect.Effect<number>,
): Effect.Effect<number> => {
  const experiment: Effect.Effect<boolean> = Effect.gen(function* () {
    const x = yield* randomInRange(x1, x2, rand);
    const y = yield* randomInRange(y1, y2, rand);
    return predicate(x, y);
  });
  return Effect.map(monteCarlo(trials, experiment), (fraction) => fraction * (x2 - x1) * (y2 - y1));
};

/** The exercise's estimate of pi: the area of the unit circle measured
 * over the square (-1, -1) to (1, 1), whose area is 4. */
export const estimatePiByIntegration = (trials: number, seed: number): Effect.Effect<number> =>
  estimateIntegral((x, y) => x * x + y * y <= 1, -1, 1, -1, 1, trials, makeRand(seed));
