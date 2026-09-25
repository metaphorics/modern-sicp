// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.6: Alyssa rewrites the square-root iteration with an
 * ordinary-procedure conditional. The statement's definitions are
 * given; the pending part is the answer: what happens when the new
 * square root runs, and why.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.6 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Averages two numbers. */
export const average = (x: number, y: number): number => (x + y) / 2;

/** A guess is improved by averaging it with the quotient x / guess. */
export const improve = (guess: number, x: number): number => average(guess, x / guess);

/** The absolute-tolerance end test: square within 0.001 of the radicand. */
export const goodEnough = (guess: number, x: number): boolean =>
  Math.abs(guess * guess - x) < 0.001;

/** Alyssa's ordinary-procedure conditional: eager in all three arguments. */
export const newIf = <T>(predicate: boolean, thenClause: T, elseClause: T): T =>
  predicate ? thenClause : elseClause;

/** The square-root iteration rewritten with newIf. */
export const sqrtIterNewIf = (guess: number, x: number): number =>
  newIf(goodEnough(guess, x), guess, sqrtIterNewIf(improve(guess, x), x));

export function ex_1_06(): string {
  throw new PendingSolution();
}
