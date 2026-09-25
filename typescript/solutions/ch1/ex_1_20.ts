// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.20: counting remainder calls in the eager gcd.
 *
 * SICP's exercise traces (gcd 206 40) under normal order and asks how
 * many remainder operations each order performs. TypeScript evaluates
 * arguments eagerly, so no procedure here can be traced under normal
 * order; a thunk-rewriting interpreter like chapter 4's would simulate
 * it. The eager count is measured by making the remainder step bump a
 * counter in the section's gcd loop. A thunk-level simulation of the
 * normal-order trace forces the remainder 18 times for the same input,
 * against the eager loop's four: the deferred argument gets copied into
 * each successive reduction and re-forced there.
 */
export function tracedGcd(a: number, b: number): { value: number; remainderCalls: number } {
  let remainderCalls = 0;
  const remainder = (x: number, y: number): number => {
    remainderCalls += 1;
    return x % y;
  };
  let x = a;
  let y = b;
  while (y !== 0) {
    const r = remainder(x, y);
    x = y;
    y = r;
  }
  return { value: x, remainderCalls };
}
