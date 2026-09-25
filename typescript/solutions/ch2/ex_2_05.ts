// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.5: pairs of nonnegative integers as 2^a * 3^b. cons
 * multiplies the two prime powers; car and cdr recover a and b by
 * counting how many times 2 and 3 divide the encoding, which terminates
 * because 2 and 3 share no factor. The encoding leaves the 53 exact bits
 * of number at a = 53, b = 1's neighborhood, so the value stays in
 * this edition's bigint throughout.
 */
/** Encodes the pair (a, b) as the integer 2^a * 3^b. */
export const cons = (a: bigint, b: bigint): bigint => 2n ** a * 3n ** b;

/** Recovers a by counting how many times 2 divides the encoding. */
export const car = (p: bigint): bigint => {
  let n = p;
  let count = 0n;
  while (n % 2n === 0n) {
    n /= 2n;
    count += 1n;
  }
  return count;
};

/** Recovers b by counting how many times 3 divides the encoding. */
export const cdr = (p: bigint): bigint => {
  let n = p;
  let count = 0n;
  while (n % 3n === 0n) {
    n /= 3n;
    count += 1n;
  }
  return count;
};
