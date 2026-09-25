// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  makeVect,
  type Painter,
  transformPainter,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.50: define `flip-horiz` and the rotations `rotate-180`
 * and `rotate-270` with `transform-painter`. Each names the frame the
 * painter is drawn into: flip-horiz runs the first edge from `(1, 0)`
 * back to `(0, 0)` so x reads `1 - x`; rotate-180 runs both edges
 * backwards from `(1, 1)`; rotate-270 turns the picture a quarter turn
 * clockwise, mapping `(x, y)` to `(y, 1 - x)`.
 */
export const flipHorizExercise = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(1.0, 0.0), makeVect(0.0, 0.0), makeVect(1.0, 1.0));

/** The book's `rotate-180`: draws the painter upside down. */
export const rotate180Exercise = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(1.0, 1.0), makeVect(0.0, 1.0), makeVect(1.0, 0.0));

/** The book's `rotate-270`: draws the painter a quarter turn clockwise. */
export const rotate270Exercise = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(0.0, 1.0), makeVect(0.0, 0.0), makeVect(1.0, 1.0));
