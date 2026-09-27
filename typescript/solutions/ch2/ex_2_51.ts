// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  append,
  beside,
  makeVect,
  type Painter,
  rotate90,
  transformPainter,
} from "../../packages/ch2/src/02-picture-language.js";
import { rotate270Exercise } from "./ex_2_50.js";

/**
 * Exercise 2.51: `below` puts the first painter in the bottom half of
 * the frame and the second in the top half. The direct construction
 * mirrors `beside`'s transforms across the horizontal midline and
 * appends the two segment lists; the rotated construction turns the
 * `beside` arrangement a quarter turn upright - the same idea the book
 * spells out with rotate-270, beside, and rotate90.
 */
export const belowDirect = (painter1: Painter, painter2: Painter): Painter => {
  const splitPoint = makeVect(0.0, 0.5);
  const paintBottom = transformPainter(
    painter1,
    makeVect(0.0, 0.0),
    makeVect(1.0, 0.0),
    splitPoint,
  );
  const paintTop = transformPainter(painter2, splitPoint, makeVect(1.0, 0.5), makeVect(0.0, 1.0));
  return (frame) => append(paintBottom(frame), paintTop(frame));
};

/** The rotated construction: quarter-turn both painters clockwise,
 * place them side by side, and rotate the arrangement back upright. */
export const belowViaRotation = (painter1: Painter, painter2: Painter): Painter =>
  rotate90(beside(rotate270Exercise(painter1), rotate270Exercise(painter2)));
