// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.36: reported fixed-point iterations and x^x = 1000.
 *
 * The book's variant prints each approximation with display/newline; the
 * host-honest form hands each new guess to an emit callback, so the search
 * stays testable and console.log can still serve at the call site. The
 * solution of x^x = 1000 is the fixed point of x |-> log(1000)/log(x),
 * tried without damping and with one average damp, always starting at 2
 * (at 1 the first step divides by log(1) = 0).
 */
const TOLERANCE = 0.00001;

export function fixedPointReport(
  f: (x: number) => number,
  firstGuess: number,
  emit: (guess: number) => void,
): number {
  let guess = firstGuess;
  for (;;) {
    const next = f(guess);
    emit(next);
    if (Math.abs(guess - next) < TOLERANCE) {
      return next;
    }
    guess = next;
  }
}

export function solveXtoTheX(): {
  value: number;
  undampedSteps: number;
  dampedSteps: number;
} {
  const step = (x: number): number => Math.log(1000) / Math.log(x);
  const damped = (x: number): number => (x + step(x)) / 2;
  let undampedSteps = 0;
  let dampedSteps = 0;
  fixedPointReport(step, 2.0, () => {
    undampedSteps += 1;
  });
  const value = fixedPointReport(damped, 2.0, () => {
    dampedSteps += 1;
  });
  return { value, undampedSteps, dampedSteps };
}
