// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.37: continued fractions, both processes.
 *
 * The k-term fraction N1/(D1 + N2/(... + Nk/Dk)) is folded from the deepest
 * term: recursively by a helper that returns 0 past term k, iteratively by
 * a loop that runs i from k down to 1. The all-ones fraction converges to
 * 1/phi, and the artifact records how large k must be for 4-decimal
 * accuracy.
 */
export function contFrac(n: (i: number) => number, d: (i: number) => number, k: number): number {
  const termAt = (i: number): number => (i > k ? 0 : n(i) / (d(i) + termAt(i + 1)));
  return termAt(1);
}

export function contFracIter(
  n: (i: number) => number,
  d: (i: number) => number,
  k: number,
): number {
  let acc = 0;
  for (let i = k; i >= 1; i -= 1) {
    acc = n(i) / (d(i) + acc);
  }
  return acc;
}

/** 1/phi, from the closed form of 1.2.2. */
const INV_PHI = 1 / ((1 + Math.sqrt(5)) / 2);

export function smallestKForFourDecimals(): number {
  const one = (): number => 1.0;
  let k = 1;
  for (;;) {
    if (Math.abs(contFracIter(one, one, k) - INV_PHI) < 1e-4) {
      return k;
    }
    k += 1;
  }
}
