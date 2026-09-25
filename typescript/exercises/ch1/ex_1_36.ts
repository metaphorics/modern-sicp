// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.36: a fixedPoint variant that reports every approximation it
 * generates, then a solution of x^x = 1000 as a fixed point of
 * x |-> log(1000)/log(x), with and without average damping.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.36 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** fixedPoint that hands each new guess to emit before continuing. */
export function fixedPointReport(
  _f: (x: number) => number,
  _firstGuess: number,
  _emit: (guess: number) => void,
): number {
  throw new PendingSolution();
}

/** The solution of x^x = 1000 and the step counts with and without damping. */
export function solveXtoTheX(): {
  value: number;
  undampedSteps: number;
  dampedSteps: number;
} {
  throw new PendingSolution();
}
