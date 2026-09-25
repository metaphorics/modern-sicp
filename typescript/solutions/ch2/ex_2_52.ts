// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  append,
  below,
  beside,
  flipHoriz,
  flipVert,
  identityOp,
  list,
  makeSegment,
  makeVect,
  type Painter,
  rightSplit,
  rotate180,
  segmentsToPainter,
  squareOfFour,
  upSplit,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.52: one change at each of the three levels of the
 * stratified design — the primitive painter's segment list, the
 * corner-split replication pattern, and the square-limit corner
 * arrangement.
 */

/** (a) The wave painter with a smile added at the primitive level. */
export const waveWithSmile = (): Painter => {
  const p = wave();
  const smile = segmentsToPainter(
    list(
      makeSegment(makeVect(0.4, 0.58), makeVect(0.47, 0.53)),
      makeSegment(makeVect(0.47, 0.53), makeVect(0.54, 0.58)),
    ),
  );
  return (frame) => append(p(frame), smile(frame));
};

/** (b) The corner-split variant: one copy each of the up-split and
 * right-split images instead of two. */
export const cornerSplitVariant = (painter: Painter, n: number): Painter => {
  if (n === 0) {
    return painter;
  }
  return beside(
    below(painter, upSplit(painter, n - 1)),
    below(rightSplit(painter, n - 1), cornerSplitVariant(painter, n - 1)),
  );
};

/** (c) The square-limit variant: the corners assembled in a different
 * pattern by reordering `square-of-four`'s arguments. */
export const squareLimitVariant = (painter: Painter, n: number): Painter =>
  squareOfFour(rotate180, flipVert, identityOp, flipHoriz)(cornerSplitVariant(painter, n));
