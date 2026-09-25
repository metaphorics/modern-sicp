// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.7: the relative-tolerance end test.
 *
 * The test watches how the guess changes and stops when the change is a
 * small fraction of the guess itself, so the tolerance scales with the
 * radicand: very small roots no longer drown in an absolute 0.001, and
 * very large ones are not asked for a precision the float cannot name.
 * The fixed point 0 is handled outright, the one input whose change
 * ratio never shrinks.
 */
function sqrtIterRelative(guess: number, previous: number, x: number): number {
  return Math.abs(guess - previous) < 0.001 * guess
    ? guess
    : sqrtIterRelative((guess + x / guess) / 2, guess, x);
}

export function sqrtRelative(x: number): number {
  if (x === 0) {
    return 0;
  }
  return sqrtIterRelative(1.0, 0.0, x);
}
