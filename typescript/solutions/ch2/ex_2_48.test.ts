// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { addVect2, scaleVect2, type Vect2 } from "./ex_2_46.js";
import { edge1FrameList, edge2FrameList, makeFrameList, originFrameList } from "./ex_2_47.js";
import { endSegment2, makeSegment2, startSegment2 } from "./ex_2_48.js";

/** The book's frame-coord-map over tuple vectors: Origin + x*Edge1 +
 * y*Edge2, built on exercise 2.46 and 2.47 pieces. */
const coordMapOf =
  (origin: Vect2, e1: Vect2, e2: Vect2) =>
  (v: Vect2): Vect2 =>
    addVect2(origin, addVect2(scaleVect2(v[0], e1), scaleVect2(v[1], e2)));

describe("exercise 2.48", () => {
  const segment = makeSegment2([0, 0], [1, 1]);

  it("make-segment pairs the endpoints and the selectors read them back", () => {
    expect(segment).toStrictEqual([
      [0, 0],
      [1, 1],
    ]);
    expect(startSegment2(segment)).toStrictEqual([0, 0]);
    expect(endSegment2(segment)).toStrictEqual([1, 1]);
  });

  it("endpoints mapped through the 2.47 list frame keep identity-frame values", () => {
    const frame = makeFrameList([0, 0], [1, 0], [0, 1]);
    const m = coordMapOf(originFrameList(frame), edge1FrameList(frame), edge2FrameList(frame));
    expect(m(startSegment2(segment))).toStrictEqual([0, 0]);
    expect(m(endSegment2(segment))).toStrictEqual([1, 1]);
  });

  it("a displaced frame moves the endpoints by origin + x*edge1 + y*edge2", () => {
    const frame = makeFrameList([1, 2], [1, 0], [0, 1]);
    const m = coordMapOf(originFrameList(frame), edge1FrameList(frame), edge2FrameList(frame));
    expect(m(startSegment2(segment))).toStrictEqual([1, 2]);
    expect(m(endSegment2(segment))).toStrictEqual([2, 3]);
  });
});
