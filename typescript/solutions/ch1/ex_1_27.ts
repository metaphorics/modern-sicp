// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.27: the Carmichael numbers of footnote 47, tried.
 *
 * The check is the Fermat congruence a^n === a (mod n) for every
 * 1 <= a < n, using the section's expmod. All six Carmichael numbers
 * pass the congruence for every witness - they are composite, yet no
 * a below n exposes them, which is precisely why the randomized
 * fastPrime of this section returns true for them.
 */
export const expmod = (base: number, exp: number, m: number): number => {
  if (exp === 0) {
    return 1;
  }
  if (exp % 2 === 0) {
    const squared = (x: number): number => x * x;
    return squared(expmod(base, exp / 2, m)) % m;
  }
  return (base * expmod(base, exp - 1, m)) % m;
};

export const passesFermatForAllA = (n: number): boolean => {
  for (let a = 1; a < n; a += 1) {
    if (expmod(a, n, n) !== a) {
      return false;
    }
  }
  return true;
};

export const carmichaelNumbers = [561, 1105, 1729, 2465, 2821, 6601];
