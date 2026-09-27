// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.1: a sign-normalizing makeRat.
 *
 * The constructor reduces with gcd exactly as the section's fix does, and
 * then decides where the sign lives from the *reduced* denominator: if it
 * is negative, both parts flip, so a positive rational has two positive
 * parts and a negative one has a negative numerator only. Deciding after
 * the reduction matters: the unreduced inputs 2/-4 and -2/-4 both reduce
 * with a positive gcd, and the denominator alone tells the final sign.
 */
function gcd(a: bigint, b: bigint): bigint {
  let x = a < 0n ? -a : a;
  let y = b < 0n ? -b : b;
  while (y !== 0n) {
    const r = x % y;
    x = y;
    y = r;
  }
  return x;
}

/** Builds the reduced rational n/d with the sign on the numerator alone. */
export function makeRatNormalized(n: bigint, d: bigint): readonly [bigint, bigint] {
  const g = gcd(n, d);
  const num = n / g;
  const den = d / g;
  return den < 0n ? [-num, -den] : [num, den];
}

/** Renders a rational as numerator/denominator. */
export function printRat(x: readonly [bigint, bigint]): string {
  return `${x[0]}/${x[1]}`;
}
