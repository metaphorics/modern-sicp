// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.18: the Russian peasant method as a loop.
 *
 * State (x, y, acc) with x * y + acc invariant: halving an even y
 * doubles x, an odd y moves one x into acc. When y reaches 0 the answer
 * is acc. Written as an explicit loop - the book's tail-recursive
 * formulation buys no constant space on Node, which collapses no tail
 * calls; the honest iterative shape here is the loop, with the book's
 * state transformation as its body.
 */
export const double = (x: number): number => x + x;

export const halve = (x: number): number => x / 2;

export function timesIter(a: number, b: number): number {
  let x = a;
  let y = b;
  let acc = 0;
  while (y > 0) {
    if (y % 2 === 0) {
      x = double(x);
      y = halve(y);
    } else {
      acc += x;
      y -= 1;
    }
  }
  return acc;
}

export function timesIterStates(
  a: number,
  b: number,
): Array<{ acc: number; x: number; y: number }> {
  const states: Array<{ acc: number; x: number; y: number }> = [];
  let x = a;
  let y = b;
  let acc = 0;
  while (y > 0) {
    states.push({ acc, x, y });
    if (y % 2 === 0) {
      x = double(x);
      y = halve(y);
    } else {
      acc += x;
      y -= 1;
    }
  }
  states.push({ acc, x, y });
  return states;
}
