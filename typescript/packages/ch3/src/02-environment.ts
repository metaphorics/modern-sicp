// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.2

/**
 * The environment model of evaluation, spelled in the edition's idiom.
 * The book's procedure objects are function values: each one carries
 * the bindings it was created next to (its environment part) together
 * with its body (its code). Nothing here needs an environment data
 * structure yet; chapter 4's evaluator builds the book's frames
 * explicitly (`Env` in `packages/ch4/src/core.ts`) and cites this
 * section as the model it realizes.
 */

// ---------------------------------------------------------------------
// 3.2.1 The Rules for Evaluation
// ---------------------------------------------------------------------

/** The book's `square`, declared as a function. A function declaration
 * binds its name in the frame that runs it and captures nothing here,
 * so its environment part is the global frame. */
export function square(x: number): number {
  return x * x;
}

/** The same procedure object built by evaluating an arrow function:
 * the edition's spelling of the book's implicit `lambda`, bound to a
 * fresh name because a second `square` in one scope is a syntax error.
 */
export const squareFn = (x: number): number => x * x;

// ---------------------------------------------------------------------
// 3.2.2 Applying Simple Procedures
// ---------------------------------------------------------------------

/** The book's `sum-of-squares`: each call builds a frame binding both
 * parameters, and the two `square` calls inside it each build their
 * own, so the three `x` bindings of the book's diagram live in three
 * different frames. */
export const sumOfSquares = (x: number, y: number): number => square(x) + square(y);

/** The book's `f`: applying it to 5 creates the frame for `a`, and the
 * argument expressions `a + 1` and `a * 2` are evaluated there before
 * `sumOfSquares` is applied to 6 and 10. */
export const f = (a: number): number => sumOfSquares(a + 1, a * 2);

// ---------------------------------------------------------------------
// 3.2.4 Internal Definitions
// ---------------------------------------------------------------------

/** The book's `average` (from 1.1.7), used by `improve`. */
export const average = (a: number, b: number): number => (a + b) / 2;

/** The book's `sqrt` with internal definitions: `goodEnough`,
 * `improve`, and `sqrtIter` are procedure objects created in the frame
 * the call to `sqrt` builds, so their names never touch the global
 * frame, and each reaches the `x` of that frame as a free variable.
 * Function declarations hoist within the body, so `sqrtIter` may name
 * helpers declared after it, exactly as Scheme's internal `define`s
 * may; a `const`-bound arrow would not. */
export const sqrt = (x: number): number => {
  function goodEnough(guess: number): boolean {
    return Math.abs(square(guess) - x) < 0.001;
  }
  function improve(guess: number): number {
    return average(guess, x / guess);
  }
  function sqrtIter(guess: number): number {
    return goodEnough(guess) ? guess : sqrtIter(improve(guess));
  }
  return sqrtIter(1);
};
