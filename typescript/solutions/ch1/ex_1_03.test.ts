// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { sumOfSquaresOfTwoLarger } from "./ex_1_03.js";

const permutationsOf = (a: number, b: number, c: number): [number, number, number][] => [
  [a, b, c],
  [a, c, b],
  [b, a, c],
  [b, c, a],
  [c, a, b],
  [c, b, a],
];

describe("exercise 1.3", () => {
  it("the two larger of 1, 2, 3 square-sum to 13", () => {
    expect(sumOfSquaresOfTwoLarger(1, 2, 3)).toBe(13);
    expect(sumOfSquaresOfTwoLarger(3, 2, 1)).toBe(13);
  });

  it("equal inputs count as their share of the two larger", () => {
    expect(sumOfSquaresOfTwoLarger(2, 2, 3)).toBe(13);
    expect(sumOfSquaresOfTwoLarger(5, 5, 5)).toBe(50);
    expect(sumOfSquaresOfTwoLarger(0, 0, 0)).toBe(0);
  });

  it("every ordering of three generated numbers gives the same sum", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: -1000, max: 1000 }),
        (a, b, c) => {
          const results = permutationsOf(a, b, c).map((p) =>
            sumOfSquaresOfTwoLarger(p[0], p[1], p[2]),
          );
          for (const result of results) {
            expect(result).toBe(results[0]);
          }
        },
      ),
    );
  });

  it("the sum equals all three squares minus the smallest squared", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: -1000, max: 1000 }),
        (a, b, c) => {
          const smallest = Math.min(a, b, c);
          expect(sumOfSquaresOfTwoLarger(a, b, c)).toBe(
            a * a + b * b + c * c - smallest * smallest,
          );
        },
      ),
    );
  });
});
