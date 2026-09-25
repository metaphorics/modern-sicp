// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  endSegment,
  type List,
  length,
  rightSplit,
  type Segment,
  showList,
  showVect,
  startSegment,
  unitSquare,
  upSplit as upSplitSection,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";
import { rightSplitViaSplit, upSplitViaSplit } from "./ex_2_45.js";

/** Renders every segment with its endpoints, in list order: showList
 * cannot see inside a segment object, so exact agreement walks the
 * list. */
const draw = (segments: List<Segment>): string => {
  const parts: string[] = [];
  for (let rest = segments; rest._tag === "Cons"; rest = rest.tail) {
    parts.push(`${showVect(startSegment(rest.head))}->${showVect(endSegment(rest.head))}`);
  }
  return parts.join(" ");
};

describe("exercise 2.45", () => {
  it("right-split via split matches the module's at depth 2", () => {
    expect(showList(rightSplitViaSplit(wave(), 2)(unitSquare))).toBe(
      showList(rightSplit(wave(), 2)(unitSquare)),
    );
    expect(draw(rightSplitViaSplit(wave(), 2)(unitSquare))).toBe(
      draw(rightSplit(wave(), 2)(unitSquare)),
    );
  });

  it("up-split via split matches the module's at depth 2", () => {
    expect(showList(upSplitViaSplit(wave(), 2)(unitSquare))).toBe(
      showList(upSplitSection(wave(), 2)(unitSquare)),
    );
    expect(draw(upSplitViaSplit(wave(), 2)(unitSquare))).toBe(
      draw(upSplitSection(wave(), 2)(unitSquare)),
    );
  });

  it("depth zero returns the painter itself", () => {
    expect(showList(rightSplitViaSplit(wave(), 0)(unitSquare))).toBe(showList(wave()(unitSquare)));
    expect(length(upSplitViaSplit(wave(), 0)(unitSquare))).toBe(18);
  });
});
