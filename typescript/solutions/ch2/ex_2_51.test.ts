// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  below,
  endSegment,
  type List,
  list,
  makeSegment,
  makeVect,
  rogers,
  type Segment,
  segmentsToPainter,
  showList,
  showVect,
  startSegment,
  unitSquare,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";
import { belowDirect, belowViaRotation } from "./ex_2_51.js";

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

/** A painter for the bottom half: one segment low in the unit square. */
const low = segmentsToPainter(list(makeSegment(makeVect(0.1, 0.2), makeVect(0.4, 0.3))));

/** A painter for the top half: one segment high in the unit square. */
const high = segmentsToPainter(list(makeSegment(makeVect(0.1, 0.7), makeVect(0.4, 0.8))));

describe("exercise 2.51", () => {
  it("the direct construction puts the first painter at the bottom", () => {
    expect(draw(belowDirect(low, high)(unitSquare))).toBe(
      "(0.1, 0.1)->(0.4, 0.15) (0.1, 0.85)->(0.4, 0.9)",
    );
  });

  it("both constructions match the module's below, segment for segment", () => {
    expect(showList(belowDirect(low, high)(unitSquare))).toBe(
      showList(below(low, high)(unitSquare)),
    );
    expect(showList(belowViaRotation(low, high)(unitSquare))).toBe(
      showList(below(low, high)(unitSquare)),
    );
    expect(draw(belowDirect(low, high)(unitSquare))).toBe(draw(below(low, high)(unitSquare)));
    expect(draw(belowViaRotation(low, high)(unitSquare))).toBe(draw(below(low, high)(unitSquare)));
  });

  it("the three spellings agree on wave over rogers too", () => {
    const fromModule = showList(below(wave(), rogers())(unitSquare));
    expect(showList(belowDirect(wave(), rogers())(unitSquare))).toBe(fromModule);
    expect(showList(belowViaRotation(wave(), rogers())(unitSquare))).toBe(fromModule);
    expect(draw(belowDirect(wave(), rogers())(unitSquare))).toBe(
      draw(below(wave(), rogers())(unitSquare)),
    );
    expect(draw(belowViaRotation(wave(), rogers())(unitSquare))).toBe(
      draw(below(wave(), rogers())(unitSquare)),
    );
  });
});
