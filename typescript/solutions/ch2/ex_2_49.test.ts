// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  endSegment,
  type List,
  length,
  type Segment,
  showList,
  showVect,
  startSegment,
  unitSquare,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";
import { crossPainter, diamond, outline, waveFromSegments } from "./ex_2_49.js";

/** Renders every segment with its endpoints, in list order. */
const draw = (segments: List<Segment>): string => {
  const parts: string[] = [];
  for (let rest = segments; rest._tag === "Cons"; rest = rest.tail) {
    parts.push(`${showVect(startSegment(rest.head))}->${showVect(endSegment(rest.head))}`);
  }
  return parts.join(" ");
};

describe("exercise 2.49", () => {
  it("a. the outline draws the frame border counterclockwise", () => {
    const painter = outline();
    expect(length(painter(unitSquare))).toBe(4);
    expect(draw(painter(unitSquare))).toBe(
      "(0, 0)->(1, 0) (1, 0)->(1, 1) (1, 1)->(0, 1) (0, 1)->(0, 0)",
    );
  });

  it("b. the diagonals cross the frame corner to corner", () => {
    const painter = crossPainter();
    expect(length(painter(unitSquare))).toBe(2);
    expect(draw(painter(unitSquare))).toBe("(0, 0)->(1, 1) (0, 1)->(1, 0)");
  });

  it("c. the diamond connects the side midpoints, starting at (0.5, 0)", () => {
    const painter = diamond();
    expect(length(painter(unitSquare))).toBe(4);
    expect(draw(painter(unitSquare))).toBe(
      "(0.5, 0)->(1, 0.5) (1, 0.5)->(0.5, 1) (0.5, 1)->(0, 0.5) (0, 0.5)->(0.5, 0)",
    );
  });

  it("d. the wave rebuilt from waveSegments equals the module's wave", () => {
    expect(showList(waveFromSegments()(unitSquare))).toBe(showList(wave()(unitSquare)));
    expect(draw(waveFromSegments()(unitSquare))).toBe(draw(wave()(unitSquare)));
  });
});
