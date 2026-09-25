// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  frameCoordMap,
  makeVect as moduleVect,
  unitSquare,
} from "../../packages/ch2/src/02-picture-language.js";
import { addVect2, scaleVect2, type Vect2 } from "./ex_2_46.js";
import {
  edge1FrameList,
  edge1FramePair,
  edge2FrameList,
  edge2FramePair,
  makeFrameList,
  makeFramePair,
  originFrameList,
  originFramePair,
} from "./ex_2_47.js";

/** The book's frame-coord-map over tuple vectors: Origin + x*Edge1 +
 * y*Edge2, the same arithmetic the module's record-vector version
 * performs. */
const coordMapOf =
  (origin: Vect2, e1: Vect2, e2: Vect2) =>
  (v: Vect2): Vect2 =>
    addVect2(origin, addVect2(scaleVect2(v[0], e1), scaleVect2(v[1], e2)));

describe("exercise 2.47", () => {
  const origin: Vect2 = [0, 0];
  const e1: Vect2 = [1, 0];
  const e2: Vect2 = [0, 1];

  it("both representations answer the same three selectors", () => {
    const asList = makeFrameList(origin, e1, e2);
    const asPair = makeFramePair(origin, e1, e2);
    expect(originFrameList(asList)).toStrictEqual([0, 0]);
    expect(edge1FrameList(asList)).toStrictEqual([1, 0]);
    expect(edge2FrameList(asList)).toStrictEqual([0, 1]);
    expect(originFramePair(asPair)).toStrictEqual([0, 0]);
    expect(edge1FramePair(asPair)).toStrictEqual([1, 0]);
    expect(edge2FramePair(asPair)).toStrictEqual([0, 1]);
  });

  it("both representations drive the frame coordinate map identically", () => {
    const asList = makeFrameList(origin, e1, e2);
    const asPair = makeFramePair(origin, e1, e2);
    const fromList = coordMapOf(
      originFrameList(asList),
      edge1FrameList(asList),
      edge2FrameList(asList),
    );
    const fromPair = coordMapOf(
      originFramePair(asPair),
      edge1FramePair(asPair),
      edge2FramePair(asPair),
    );
    expect(fromList([0.5, 0.5])).toStrictEqual([0.5, 0.5]);
    expect(fromPair([0.5, 0.5])).toStrictEqual([0.5, 0.5]);
    expect(frameCoordMap(unitSquare)(moduleVect(0.5, 0.5))).toStrictEqual({ x: 0.5, y: 0.5 });
  });
});
