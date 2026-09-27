// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.14: Louis Reasoner installs the host's map as an evaluator
 * primitive. The demand: show by running code which calls work and which
 * fail (a map that cannot call evaluator procedures dies on compound
 * procedures), then build Eva Lu Ator's version, whose map applies
 * evaluator procedures through the evaluator's own apply, and show every
 * call works there.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.14 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Works under Louis's install: car is a primitive the host can reach. */
export const louisCarCall = "(map car '((1 2) (3 4)))";

/** Fails under Louis's install: the lambda is an evaluator closure. */
export const louisLambdaCall = "(map (lambda (p) p) '((9)))";

/** The same two calls, the yardstick Eva's map must satisfy. */
export const evaLambdaCall = "(map (lambda (n) (* n n)) '(1 2 3))";

export function ex_4_14(): string {
  throw new PendingSolution();
}
