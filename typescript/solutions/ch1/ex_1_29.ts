// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.29: Simpson's Rule integration.
 *
 * The panel count n must be even; h = (b - a)/n and the y_k of the statement
 * are f(a + k*h). The weights 1, 4, 2, ..., 2, 4, 1 are folded by a loop
 * over k: the loop shape of this edition costs nothing here and would let a
 * 50000-panel run through where a self-call per panel would not.
 */
export function simpson(f: (x: number) => number, a: number, b: number, n: number): number {
  const h = (b - a) / n;
  let total = 0;
  for (let k = 0; k <= n; k += 1) {
    const weight = k === 0 || k === n ? 1 : k % 2 === 1 ? 4 : 2;
    total += weight * f(a + k * h);
  }
  return (h / 3) * total;
}
