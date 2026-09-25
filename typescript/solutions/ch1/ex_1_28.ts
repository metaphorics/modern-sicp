// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.28: the Miller-Rabin test.
 *
 * The squaring step of expmod watches for a nontrivial square root of
 * 1 modulo n - a number other than 1 or n - 1 whose square is 1 - and
 * signals it by returning 0, per the statement's hint. If n is prime
 * no such root exists, so a 0 anywhere proves compositeness; for odd
 * composites at least half the witnesses reveal one, which is why this
 * test cannot be fooled by Carmichael numbers. Witnesses run through
 * the same rng.random shape as fermatTest; the fixed-witness variant
 * makes the transcript replayable under the deterministic-testing rule.
 */
export const mrExpmod = (base: number, exp: number, m: number): number => {
  const squareUntilCheck = (x: number): number => x * x;
  if (exp === 0) {
    return 1;
  }
  if (exp % 2 === 0) {
    const half = mrExpmod(base, exp / 2, m);
    if (half !== 1 && half !== m - 1 && squareUntilCheck(half) % m === 1) {
      return 0;
    }
    return squareUntilCheck(half) % m;
  }
  return (base * mrExpmod(base, exp - 1, m)) % m;
};

/** One Miller-Rabin trial with an explicit witness a. */
export const millerRabinWitness = (n: number, a: number): boolean => {
  const witness = mrExpmod(a % n, n - 1, n);
  return witness !== 0 && witness === 1 % n;
};

/** The randomized analog of fermatTest: `times` trials, seeded source. */
export function millerRabin(
  n: number,
  times: number,
  rng: { random(next: number): number },
): boolean {
  let remaining = times;
  while (remaining > 0) {
    const a = 1 + rng.random(n - 1);
    if (!millerRabinWitness(n, a)) {
      return false;
    }
    remaining -= 1;
  }
  return true;
}
