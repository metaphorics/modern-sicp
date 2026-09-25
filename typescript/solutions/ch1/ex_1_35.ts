// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.35: the golden ratio as a fixed point.
 *
 * If x = 1 + 1/x then x^2 = x + 1, which is exactly the defining equation
 * of the golden ratio of 1.2.2 (the positive root of x^2 = x + 1). So the
 * transformation x |-> 1 + 1/x has phi as a fixed point, and the section's
 * damped-free fixedPoint search converges on it from 1.
 */
const TOLERANCE = 0.00001;

const fixedPoint = (f: (x: number) => number, firstGuess: number): number => {
  let guess = firstGuess;
  for (;;) {
    const next = f(guess);
    if (Math.abs(guess - next) < TOLERANCE) {
      return next;
    }
    guess = next;
  }
};

export function goldenRatio(): number {
  return fixedPoint((x) => 1 + 1 / x, 1.0);
}
