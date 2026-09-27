// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.30: the summation as a loop.
 *
 * The book fills in an `iter` skeleton; on Node a self-call in last
 * position takes a frame, so the iterative process is written directly as a
 * `while` loop over the state - the running result and the current value of
 * `a` - exactly the state the skeleton's `iter` would have carried.
 */
export function sumIter(
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  let result = 0;
  let x = a;
  while (x <= b) {
    result = term(x) + result;
    x = next(x);
  }
  return result;
}
