// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { newIf, sqrtIter, sqrtIterNewIf } from "./ex_1_06.js";

/** The demo comparisons of Eva's demonstration, typed as number comparisons. */
const eq = (a: number, b: number): boolean => a === b;

describe("exercise 1.6", () => {
  it("new-if answers correctly when the branches are plain values", () => {
    expect(newIf(eq(2, 3), 0, 5)).toBe(5);
    expect(newIf(eq(1, 1), 0, 5)).toBe(0);
  });

  it("Alyssa's rewrite diverges: the host ends it with a stack overflow", () => {
    expect(() => sqrtIterNewIf(1.0, 2)).toThrow(RangeError);
  });

  it("the control iteration with the special-form conditional converges", () => {
    expect(sqrtIter(1.0, 2)).toBe(1.4142156862745097);
  });
});
