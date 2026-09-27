// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  list,
  makeSegment,
  makeVect,
  type Painter,
  type Segment,
  segmentsToPainter,
  waveSegments,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.49: the picture language's primitive painters, each a
 * segment list handed to the section's `segments->painter` in
 * unit-square coordinates. The segment types are the section's own, so
 * these painters interoperate with every combinator the language
 * already defines.
 */

/** Builds a unit-square segment from four endpoint coordinates. */
const seg = (x1: number, y1: number, x2: number, y2: number): Segment =>
  makeSegment(makeVect(x1, y1), makeVect(x2, y2));

/** a. The outline of the frame: the four border segments walked
 * counterclockwise from the origin. */
export const outline = (): Painter =>
  segmentsToPainter(list(seg(0, 0, 1, 0), seg(1, 0, 1, 1), seg(1, 1, 0, 1), seg(0, 1, 0, 0)));

/** b. The diagonals of the frame: the X connecting opposite corners. */
export const crossPainter = (): Painter =>
  segmentsToPainter(list(seg(0, 0, 1, 1), seg(0, 1, 1, 0)));

/** c. The diamond connecting the midpoints of the frame's sides. */
export const diamond = (): Painter =>
  segmentsToPainter(
    list(seg(0.5, 0, 1, 0.5), seg(1, 0.5, 0.5, 1), seg(0.5, 1, 0, 0.5), seg(0, 0.5, 0.5, 0)),
  );

/** d. The wave painter, rebuilt from the section's wave segment list. */
export const waveFromSegments = (): Painter => segmentsToPainter(waveSegments());
