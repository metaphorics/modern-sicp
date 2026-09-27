// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { below, beside, type Painter } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.44: write `up-split`, the mirror of `right-split`. Where
 * right-split places the painter beside two half-size copies stacked
 * one below the other, up-split places the painter below two half-size
 * copies set side by side, so the pattern branches upwards instead of
 * towards the right. The recursive skeleton is the same: depth 0 is the
 * painter itself, deeper depths combine it with the next smaller result.
 */
export const upSplit = (painter: Painter, n: number): Painter => {
  if (n === 0) {
    return painter;
  }
  const smaller = upSplit(painter, n - 1);
  return below(painter, beside(smaller, smaller));
};
