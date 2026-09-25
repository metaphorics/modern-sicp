// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { aPlusAbsB } from "./ex_1_04.js";

describe("exercise 1.4", () => {
  it("positive b takes the addition, nonpositive b the subtraction", () => {
    expect(aPlusAbsB(3, 5)).toBe(8);
    expect(aPlusAbsB(3, -5)).toBe(8);
    expect(aPlusAbsB(2, 0)).toBe(2);
    expect(aPlusAbsB(0, -7)).toBe(7);
  });

  it("the procedure computes a + |b| for generated inputs", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: -1000, max: 1000 }),
        (a, b) => {
          expect(aPlusAbsB(a, b)).toBe(a + Math.abs(b));
        },
      ),
    );
  });
});
