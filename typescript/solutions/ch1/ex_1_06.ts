// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.6: the new-if experiment, answered empirically.
 *
 * newIf is an ordinary procedure, so the host evaluates all three of its
 * arguments before the call runs. Alyssa's rewrite therefore evaluates
 * the recursive branch on every iteration, the recursion never bottoms
 * out, and the host ends the process with a stack-overflow RangeError.
 * The special-form conditional differs precisely in that it evaluates
 * only the branch it takes.
 */
export const average = (x: number, y: number): number => (x + y) / 2;

export const improve = (guess: number, x: number): number => average(guess, x / guess);

export const goodEnough = (guess: number, x: number): boolean =>
  Math.abs(guess * guess - x) < 0.001;

export const newIf = <T>(predicate: boolean, thenClause: T, elseClause: T): T =>
  predicate ? thenClause : elseClause;

export const sqrtIterNewIf = (guess: number, x: number): number =>
  newIf(goodEnough(guess, x), guess, sqrtIterNewIf(improve(guess, x), x));

/** The section's conditional-operator iteration: the control that converges. */
export const sqrtIter = (guess: number, x: number): number =>
  goodEnough(guess, x) ? guess : sqrtIter(improve(guess, x), x);

export function ex_1_06(): string {
  return (
    "Every newIf call evaluates all three arguments first, so the recursive " +
    "branch runs on every iteration, the recursion never stops, and the host " +
    "ends the square-root computation with a stack-overflow RangeError. An if " +
    "must be special precisely so that only the taken branch is evaluated."
  );
}
