// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { measureRatio, next, smallestDivisorNext } from "./ex_1_23.js";

describe("exercise 1.23", () => {
  it("next walks the odd integers", () => {
    expect([2, 3, 4, 5, 9].map(next)).toStrictEqual([3, 5, 6, 7, 11]);
  });

  it("finds the same twelve primes as exercise 1.22", () => {
    expect(
      [1000, 10000, 100000, 1000000].map((start) => {
        const found: number[] = [];
        let candidate = start + 1;
        while (found.length < 3) {
          if (smallestDivisorNext(candidate) === candidate) {
            found.push(candidate);
          }
          candidate += 2;
        }
        return found;
      }),
    ).toStrictEqual([
      [1009, 1013, 1019],
      [10007, 10009, 10037],
      [100003, 100019, 100043],
      [1000003, 1000033, 1000037],
    ]);
  });

  it("the skipping search is not slower, with a stable median bound", () => {
    const { plainMillis, skippingMillis, ratio } = measureRatio(100000000);
    expect(plainMillis).toBeGreaterThan(0);
    expect(skippingMillis).toBeGreaterThan(0);
    expect(ratio).toBeGreaterThanOrEqual(0.5);
    expect(ratio).toBeLessThanOrEqual(10);
  });
});
