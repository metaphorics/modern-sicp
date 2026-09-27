// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { area } from "./ex_0_04.js";

describe("exercise 0.4", () => {
  it("each variant computes its own area", () => {
    expect(area({ _tag: "Circle", r: 1 })).toBe(Math.PI);
    expect(area({ _tag: "Rect", w: 2, h: 3 })).toBe(6);
    expect(area({ _tag: "Square", s: 4 })).toBe(16);
  });

  it("areas are positive and scale with the dimensions", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: 1, max: 100 }),
        fc.integer({ min: 1, max: 100 }),
        fc.integer({ min: 1, max: 100 }),
        (a, b, s) => {
          expect(area({ _tag: "Rect", w: a, h: b })).toBe(a * b);
          expect(area({ _tag: "Square", s })).toBe(s * s);
        },
      ),
    );
  });
});
