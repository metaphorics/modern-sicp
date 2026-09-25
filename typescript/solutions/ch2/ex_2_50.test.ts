// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  endSegment,
  flipHoriz,
  type List,
  list,
  makeSegment,
  makeVect,
  rotate90,
  rotate180,
  type Segment,
  segmentsToPainter,
  showList,
  showVect,
  startSegment,
  unitSquare,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";
import { flipHorizExercise, rotate180Exercise, rotate270Exercise } from "./ex_2_50.js";

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

/** A probe painter at dyadic coordinates: every frame computation on it
 * is exact, so orientation checks pin byte-for-byte strings. */
const probe = segmentsToPainter(list(makeSegment(makeVect(0.25, 0.125), makeVect(0.375, 0.5))));

describe("exercise 2.50", () => {
  it("flip-horiz mirrors the painter exactly as the module's does", () => {
    expect(showList(flipHorizExercise(wave())(unitSquare))).toBe(
      showList(flipHoriz(wave())(unitSquare)),
    );
    expect(draw(flipHorizExercise(wave())(unitSquare))).toBe(draw(flipHoriz(wave())(unitSquare)));
  });

  it("rotate-180 turns the painter upside down exactly as the module's does", () => {
    expect(showList(rotate180Exercise(wave())(unitSquare))).toBe(
      showList(rotate180(wave())(unitSquare)),
    );
    expect(draw(rotate180Exercise(wave())(unitSquare))).toBe(draw(rotate180(wave())(unitSquare)));
  });

  it("rotate-270 maps (x, y) to (y, 1 - x)", () => {
    expect(draw(rotate270Exercise(probe)(unitSquare))).toBe("(0.125, 0.75)->(0.5, 0.625)");
  });

  it("rotate-270 applied twice is rotate-180", () => {
    expect(draw(rotate270Exercise(rotate270Exercise(probe))(unitSquare))).toBe(
      draw(rotate180Exercise(probe)(unitSquare)),
    );
  });

  it("rotate-270 applied three times is one rotate-90, not a flip", () => {
    expect(draw(rotate270Exercise(rotate270Exercise(rotate270Exercise(probe)))(unitSquare))).toBe(
      draw(rotate90(probe)(unitSquare)),
    );
  });

  it("rotate-270 applied four times restores the painter", () => {
    expect(
      draw(
        rotate270Exercise(rotate270Exercise(rotate270Exercise(rotate270Exercise(probe))))(
          unitSquare,
        ),
      ),
    ).toBe(draw(probe(unitSquare)));
  });
});
