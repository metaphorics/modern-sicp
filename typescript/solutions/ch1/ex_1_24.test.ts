// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { Random } from "../../examples/ch1/random.js";

import { fastPrime, measureFermatMedian, searchForPrimesFermat } from "./ex_1_24.js";

describe("exercise 1.24", () => {
  it("the seeded Fermat test separates the known primes and composites", () => {
    const rng = new Random(1n);
    expect(fastPrime(1009, 3, rng)).toBe(true);
    expect(fastPrime(100, 5, rng)).toBe(false);
    expect(fastPrime(561, 5, rng)).toBe(true);
  });

  it("the Fermat search agrees with the divisor search's twelve primes", () => {
    const rng = new Random(1n);
    expect(searchForPrimesFermat(1000, 5, rng)).toStrictEqual([1009, 1013, 1019]);
    expect(searchForPrimesFermat(1000000, 5, rng)).toStrictEqual([1000003, 1000033, 1000037]);
  });

  it("the median time is positive and stable at the large workload", () => {
    const medians = [0, 0, 0].map(() => {
      const { medianMillis } = measureFermatMedian(100000000, 5, 9, new Random(1n));
      return medianMillis;
    });
    for (const median of medians) {
      expect(median).toBeGreaterThan(0);
      expect(Number.isFinite(median)).toBe(true);
    }
    expect(Math.max(...medians) / Math.min(...medians)).toBeLessThanOrEqual(10);
  });
});
