// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.40: cubic for Newton's method.
 *
 * cubic(a, b, c) is the function x |-> x^3 + ax^2 + bx + c, and
 * newtonsMethod(cubic(a, b, c), guess) is a zero of it. The Newton
 * machinery below is the section's own deriv, newtonTransform, and
 * fixedPoint of 1.3.4, restated to keep the solution self-contained.
 */
const TOLERANCE = 0.00001;
const DX = 0.00001;

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

const deriv =
  (g: (x: number) => number): ((x: number) => number) =>
  (x) =>
    (g(x + DX) - g(x)) / DX;

const newtonTransform =
  (g: (x: number) => number): ((x: number) => number) =>
  (x) =>
    x - g(x) / deriv(g)(x);

/** Newton's method of 1.3.4: the fixed-point search on the Newton transform. */
export function newtonsMethod(g: (x: number) => number, guess: number): number {
  return fixedPoint(newtonTransform(g), guess);
}

/** The cubic x^3 + ax^2 + bx + c as a one-argument function. */
export function cubic(a: number, b: number, c: number): (x: number) => number {
  return (x: number): number => x * x * x + a * x * x + b * x + c;
}
