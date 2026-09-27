// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { eApprox } from "./ex_1_38.js";

describe("exercise 1.38", () => {
  it("the denominator rule produces 1, 2, 1, 1, 4, 1, 1, 6, 1, 1", () => {
    const eulerD = (i: number): number => ((i + 1) % 3 === 0 ? (2 * (i + 1)) / 3 : 1);
    expect([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map(eulerD)).toStrictEqual([
      1, 2, 1, 1, 4, 1, 1, 6, 1, 1,
    ]);
  });

  it("thirteen terms carry nine correct digits of e", () => {
    expect(eApprox(13)).toBe(2.718281828735696);
    expect(eApprox(13)).toBeCloseTo(Math.E, 9);
  });

  it("twenty terms land on the host's own double for e", () => {
    expect(eApprox(20)).toBe(Math.E);
  });

  it("five terms are already close", () => {
    expect(eApprox(5)).toBe(2.71875);
  });
});
