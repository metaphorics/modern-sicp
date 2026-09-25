// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.46: iterativeImprove takes a good-enough test and an improve
 * step and returns a function that improves a guess until it is good
 * enough. An arrow cannot call itself by name and Node gives no tail-call
 * guarantee, so the improvement runs as a while loop inside the returned
 * function. sqrt of 1.1.7 and fixedPoint of 1.3.3 are rewritten through it.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.46 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The improver: keeps improving the guess until goodEnough says stop. */
export function iterativeImprove(
  _goodEnough: (guess: number) => boolean,
  _improve: (guess: number) => number,
): (guess: number) => number {
  throw new PendingSolution();
}

/** The square root of 1.1.7, rewritten through iterativeImprove. */
export function sqrtIterativeImprove(_x: number): number {
  throw new PendingSolution();
}

/** The fixed-point search of 1.3.3, rewritten through iterativeImprove. */
export function fixedPointIterativeImprove(_f: (x: number) => number, _firstGuess: number): number {
  throw new PendingSolution();
}
