// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.8: cube roots by Newton's method.
 *
 * The improvement formula (x / y^2 + 2y) / 3 replaces the square-root
 * improve; the end test is exercise 1.7's relative one, since the same
 * extremes argument applies. The helpers are nested function
 * declarations, the block-structured shape of section 1.1.8: a function
 * declaration may be written in any order inside the body, unlike a
 * `const` arrow, which is unusable above its own declaration.
 */
export function cubeRoot(x: number): number {
  if (x === 0) {
    return 0;
  }
  function improveCube(guess: number): number {
    return (x / (guess * guess) + 2 * guess) / 3;
  }
  function goodEnoughCube(guess: number, previous: number): boolean {
    return Math.abs(guess - previous) < 0.001 * guess;
  }
  function iter(guess: number, previous: number): number {
    return goodEnoughCube(guess, previous) ? guess : iter(improveCube(guess), guess);
  }
  return iter(1.0, 0.0);
}
