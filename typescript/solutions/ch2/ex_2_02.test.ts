// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  endSegment,
  makePoint,
  makeSegment,
  midpointSegment,
  printPoint,
  startSegment,
  xPoint,
  yPoint,
} from "./ex_2_02.js";

describe("exercise 2.2", () => {
  it("the selectors read back what the constructors glued", () => {
    const p = makePoint(1, 2);
    expect(xPoint(p)).toBe(1);
    expect(yPoint(p)).toBe(2);
    const s = makeSegment(makePoint(1, 2), makePoint(3, 4));
    expect(startSegment(s)).toStrictEqual([1, 2]);
    expect(endSegment(s)).toStrictEqual([3, 4]);
  });

  it("the midpoint of the segment from (1,2) to (3,4) is (2,3)", () => {
    const s = makeSegment(makePoint(1, 2), makePoint(3, 4));
    expect(midpointSegment(s)).toStrictEqual([2, 3]);
  });

  it("printPoint renders the book's format", () => {
    expect(printPoint(makePoint(2, 3))).toBe("(2,3)");
    expect(printPoint(midpointSegment(makeSegment(makePoint(-1, 0), makePoint(1, 2))))).toBe(
      "(0,1)",
    );
  });
});
