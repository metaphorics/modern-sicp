// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * The edition's seeded pseudo-random source (D31): xorshift64* (Vigna 2016).
 *
 * The state update is `x ^= x >> 12; x ^= x << 25; x ^= x >> 27` over unsigned
 * 64-bit words and the output is the updated state multiplied by
 * `0x2545F4914F6CDD1D`; `random(n)` is the word modulo `n`. The seed must be
 * nonzero. From seed 1, `random(1000)` yields 165, 517, 103, 413, 928, which
 * every edition asserts so the Monte Carlo sections agree.
 */
export class InvalidSeedError extends Error {
  constructor(seed: bigint) {
    super(`random seed must be nonzero, got ${seed}`);
    this.name = "InvalidSeedError";
  }
}

const MULTIPLIER = 0x2545f4914f6cdd1dn;
const MASK64 = (1n << 64n) - 1n;

/** Seeded generator shared by every section that draws random numbers. */
export class Random {
  #state: bigint;

  constructor(seed: bigint) {
    const masked = seed & MASK64;
    if (masked === 0n) {
      throw new InvalidSeedError(seed);
    }
    this.#state = masked;
  }

  /** Advances the state and returns the full unsigned 64-bit word. */
  nextU64(): bigint {
    let x = this.#state;
    x = (x ^ (x >> 12n)) & MASK64;
    x = (x ^ ((x << 25n) & MASK64)) & MASK64;
    x = (x ^ (x >> 27n)) & MASK64;
    this.#state = x;
    return (x * MULTIPLIER) & MASK64;
  }

  /** Returns a float in [0, 1) from the top 53 bits of the next word. */
  next(): number {
    return Number(this.nextU64() >> 11n) / 2 ** 53;
  }

  /** Returns the next word reduced modulo the positive integer `n`. */
  random(n: number): number {
    if (!(Number.isInteger(n) && n > 0)) {
      throw new RangeError(`random(n) requires a positive integer, got ${n}`);
    }
    return Number(this.nextU64() % BigInt(n));
  }
}
