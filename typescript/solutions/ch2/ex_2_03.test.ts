// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  area,
  makePoint,
  makeRectangleBaseHeight,
  makeRectangleCorners,
  perimeter,
} from "./ex_2_03.js";

describe("exercise 2.3", () => {
  it("the same 4-by-3 rectangle gives the same four numbers both ways", () => {
    const fromCorners = makeRectangleCorners(makePoint(0, 0), makePoint(4, 3));
    const fromBaseHeight = makeRectangleBaseHeight(makePoint(0, 0), 4, 3);
    for (const r of [fromCorners, fromBaseHeight]) {
      expect(perimeter(r)).toBe(14);
      expect(area(r)).toBe(12);
    }
  });

  it("the corner form is orientation-free", () => {
    const flipped = makeRectangleCorners(makePoint(4, 3), makePoint(0, 0));
    expect(perimeter(flipped)).toBe(14);
    expect(area(flipped)).toBe(12);
  });
});
