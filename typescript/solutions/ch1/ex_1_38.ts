// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.38: Euler's continued fraction for e.
 *
 * e - 2 is the continued fraction with all numerators 1 and denominators
 * 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, ...: every third denominator (i = 2, 5,
 * 8, ...) is 2(i + 1)/3 and the rest are 1. The fraction is computed with
 * the loop cont-frac of exercise 1.37, and e is 2 plus the fraction.
 */
const contFracIter = (n: (i: number) => number, d: (i: number) => number, k: number): number => {
  let acc = 0;
  for (let i = k; i >= 1; i -= 1) {
    acc = n(i) / (d(i) + acc);
  }
  return acc;
};

const one = (): number => 1.0;

const eulerD = (i: number): number => ((i + 1) % 3 === 0 ? (2 * (i + 1)) / 3 : 1);

export function eApprox(k: number): number {
  return 2 + contFracIter(one, eulerD, k);
}
