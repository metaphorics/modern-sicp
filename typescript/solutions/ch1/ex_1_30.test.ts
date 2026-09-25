// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { sumIter } from "./ex_1_30.js";

describe("exercise 1.30", () => {
  it("reproduces the section's integer sums", () => {
    expect(
      sumIter(
        (x) => x,
        1,
        (x) => x + 1,
        10,
      ),
    ).toBe(55);
    expect(
      sumIter(
        (x) => x * x * x,
        1,
        (x) => x + 1,
        10,
      ),
    ).toBe(3025);
  });

  it("the loop's pi lands one last-digit step from the recursion's", () => {
    // The loop folds left (0 + t1) + t5 ..., the recursion folds right
    // t995 + (t999 + t1000); double addition is not associative, so the
    // two orders differ in the 16th digit.
    expect(
      8 *
        sumIter(
          (x) => 1.0 / (x * (x + 2)),
          1,
          (x) => x + 4,
          1000,
        ),
    ).toBe(3.139592655589782);
    expect(
      8 *
        sumIter(
          (x) => 1.0 / (x * (x + 2)),
          1,
          (x) => x + 4,
          1000,
        ),
    ).toBeCloseTo(3.139592655589783, 14);
  });

  it("agrees with the recursive sum over a grid of ranges", () => {
    const recursiveSum = (
      term: (x: number) => number,
      a: number,
      next: (x: number) => number,
      b: number,
    ): number => (a > b ? 0 : term(a) + recursiveSum(term, next(a), next, b));
    const inc = (x: number): number => x + 1;
    for (let a = 1; a <= 5; a += 1) {
      for (let b = a; b <= a + 8; b += 1) {
        expect(sumIter((x) => x * x, a, inc, b)).toBe(recursiveSum((x) => x * x, a, inc, b));
      }
    }
  });

  it("an empty range contributes the null value 0", () => {
    expect(
      sumIter(
        (x) => x,
        5,
        (x) => x + 1,
        4,
      ),
    ).toBe(0);
  });
});
