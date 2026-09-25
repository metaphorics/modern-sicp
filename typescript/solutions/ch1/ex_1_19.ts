// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.19: Fibonacci by transform squaring, over bigint.
 *
 * Applying T_pq twice is the single transformation T_p'q' with
 * p' = p^2 + q^2 and q' = q^2 + 2pq, so T^n can be computed by
 * successive squaring: halve the count and square (p, q); otherwise
 * apply T once. The state is (a, b, p, q, count) in one explicit loop -
 * the loop shape of this edition - and every arithmetic step is bigint,
 * because Fib(93) already passes 2^53 where number stops being exact.
 */
export function fibLog(n: number): bigint {
  let a = 1n;
  let b = 0n;
  let p = 0n;
  let q = 1n;
  let count = n;
  while (count > 0) {
    if (count % 2 === 0) {
      const pSquared = p * p;
      const qSquared = q * q;
      const nextP = pSquared + qSquared;
      const nextQ = 2n * p * q + qSquared;
      p = nextP;
      q = nextQ;
      count /= 2;
    } else {
      const previousA = a;
      a = b * q + a * q + a * p;
      b = b * p + previousA * q;
      count -= 1;
    }
  }
  return b;
}
