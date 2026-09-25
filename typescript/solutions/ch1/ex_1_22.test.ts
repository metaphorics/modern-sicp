// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { searchForPrimes, timedPrimeTest } from "./ex_1_22.js";

describe("exercise 1.22", () => {
  it("finds the three smallest primes above each threshold", () => {
    expect(searchForPrimes(1000)).toStrictEqual([1009, 1013, 1019]);
    expect(searchForPrimes(10000)).toStrictEqual([10007, 10009, 10037]);
    expect(searchForPrimes(100000)).toStrictEqual([100003, 100019, 100043]);
    expect(searchForPrimes(1000000)).toStrictEqual([1000003, 1000033, 1000037]);
  });

  it("reports no timing for a composite and a nonnegative one for a prime", () => {
    expect(timedPrimeTest(100)).toBeNull();
    const timed = timedPrimeTest(101);
    expect(timed).not.toBeNull();
    expect(timed?.n).toBe(101);
    expect(timed?.elapsedMillis).toBeGreaterThanOrEqual(0);
    expect(Number.isFinite(timed?.elapsedMillis ?? Number.NaN)).toBe(true);
  });
});
