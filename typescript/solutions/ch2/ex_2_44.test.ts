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
import { upSplit } from "./ex_2_44.js";

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

describe("exercise 2.44", () => {
  it("up-split stacks the painter above two half-size copies", () => {
    expect(length(upSplit(wave(), 1)(unitSquare))).toBe(54);
  });

  it("the depth-one count mirrors right-split's", () => {
    expect(length(rightSplit(wave(), 1)(unitSquare))).toBe(54);
  });

  it("depth zero leaves the painter unchanged", () => {
    expect(length(upSplit(wave(), 0)(unitSquare))).toBe(18);
  });

  it("agrees segment-for-segment with the module's up-split at depth 3", () => {
    expect(showList(upSplit(wave(), 3)(unitSquare))).toBe(
      showList(upSplitSection(wave(), 3)(unitSquare)),
    );
    expect(draw(upSplit(wave(), 3)(unitSquare))).toBe(draw(upSplitSection(wave(), 3)(unitSquare)));
  });
});
