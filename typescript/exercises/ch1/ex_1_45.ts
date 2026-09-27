// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.45: experiments with fixedPoint, averageDamp, and repeated
 * decide how many average damps n-th roots need; the artifact reports the
 * experimental damping count and a capped damped fixed-point search whose
 * result says whether the search converged.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.45 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The smallest number of average damps that converges for the n-th root. */
export function dampsRequired(_n: number): number {
  throw new PendingSolution();
}

/** The damped fixed-point search for the n-th root of x, capped at stepCap steps. */
export function nthRoot(
  _x: number,
  _n: number,
  _damps: number,
  _stepCap?: number,
): { root: number | null; steps: number } {
  throw new PendingSolution();
}
