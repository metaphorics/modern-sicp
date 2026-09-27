// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.25: Alyssa's simplification, measured against the host.
 *
 * She is right about the mathematics and wrong about the machine. The
 * section's expmod reduces mod m at every step, so no intermediate ever
 * grows past m^2. Hers computes base^exp first: as a number that is an
 * IEEE double, inexact past 2^53 and Infinity past about 10^308, where
 * `%` yields NaN. The bigint restatement is exact at any exponent but
 * pays for arithmetic on numbers with tens of thousands of digits,
 * while the reduction-on-every-step version never touches a number
 * bigger than m^2.
 */
export const square = (x: number): number => x * x;

export const fastExpt = (b: number, n: number): number =>
  n === 0 ? 1 : isEvenN(n) ? square(fastExpt(b, n / 2)) : b * fastExpt(b, n - 1);

const isEvenN = (n: number): boolean => n % 2 === 0;

/** The section's expmod: reduces at every step, intermediates stay below m^2. */
export const expmod = (base: number, exp: number, m: number): number => {
  if (exp === 0) {
    return 1;
  }
  if (isEvenN(exp)) {
    return square(expmod(base, exp / 2, m)) % m;
  }
  return (base * expmod(base, exp - 1, m)) % m;
};

/** Alyssa's version: the whole exponential first, remainder after. */
export const expmodSimplified = (base: number, exp: number, m: number): number =>
  fastExpt(base, exp) % m;

/** fast-expt over bigint: exact at any exponent. */
export function fastExptBigint(base: bigint, exp: number): bigint {
  let result = 1n;
  let baseAcc = base;
  let expAcc = exp;
  while (expAcc > 0) {
    if (expAcc % 2 === 0) {
      baseAcc *= baseAcc;
      expAcc /= 2;
    } else {
      result *= baseAcc;
      expAcc -= 1;
    }
  }
  return result;
}

/** Alyssa's idea over bigint: correct at any exponent, but the
 * intermediates carry tens of thousands of digits. */
export const expmodSimplifiedBigint = (base: bigint, exp: number, m: bigint): bigint =>
  fastExptBigint(base, exp) % m;
