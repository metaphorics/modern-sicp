// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.46: iterative improvement as a returned function.
 *
 * iterativeImprove(goodEnough, improve) returns the function that keeps
 * improving a guess until goodEnough accepts it. An arrow cannot call
 * itself by name and Node gives no tail-call guarantee, so the map's
 * recorded shape for this exercise is a loop inside the returned
 * function: the improvement process is the while statement's state, not
 * the call stack's. Square root of 1.1.7 and the fixed-point search of
 * 1.3.3 come back as one-line applications of the abstraction.
 */
export function iterativeImprove(
  goodEnough: (guess: number) => boolean,
  improve: (guess: number) => number,
): (guess: number) => number {
  return (guess: number): number => {
    let current = guess;
    while (!goodEnough(current)) {
      current = improve(current);
    }
    return current;
  };
}

/** The square root of 1.1.7, rewritten through iterativeImprove. */
export function sqrtIterativeImprove(x: number): number {
  return iterativeImprove(
    (guess) => Math.abs(guess * guess - x) < 0.001,
    (guess) => (guess + x / guess) / 2,
  )(1.0);
}

/** The fixed-point search of 1.3.3, rewritten through iterativeImprove. */
export function fixedPointIterativeImprove(f: (x: number) => number, firstGuess: number): number {
  return iterativeImprove((guess) => Math.abs(guess - f(guess)) < 0.00001, f)(firstGuess);
}
