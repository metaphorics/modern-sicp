// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { searchForPrimesMedian } from "./ex_1_22a.js";

describe("exercise 1.22a", () => {
  it("every round still finds the pinned primes above 100,000,000", () => {
    const timing = searchForPrimesMedian(100000000);
    expect(timing.primes).toStrictEqual([100000007, 100000037, 100000039]);
    expect(timing.samples.length).toBe(9);
  });

  it("the median is positive and stable across repetitions, never pinned to a value", () => {
    const medians = [0, 0, 0].map(() => searchForPrimesMedian(100000000).medianMillis);
    for (const median of medians) {
      expect(median).toBeGreaterThan(0);
      expect(Number.isFinite(median)).toBe(true);
    }
    const spread = Math.max(...medians) / Math.min(...medians);
    expect(spread).toBeLessThanOrEqual(10);
  });
});
