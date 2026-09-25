// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { below, beside, type Painter } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.45: `right-split` and `up-split` differ only in the two
 * combining operations. `split` takes them as arguments and returns the
 * branching procedure: depth 0 is the painter itself, deeper depths are
 * `combiner(painter, subCombiner(s, s))` for the recursively built
 * smaller result `s`. Each book procedure becomes a one-line instance.
 */
export function split(
  combiner: (a: Painter, b: Painter) => Painter,
  subCombiner: (a: Painter, b: Painter) => Painter,
): (painter: Painter, n: number) => Painter {
  const branch = (painter: Painter, n: number): Painter => {
    if (n === 0) {
      return painter;
    }
    const smaller = branch(painter, n - 1);
    return combiner(painter, subCombiner(smaller, smaller));
  };
  return branch;
}

/** The book's `right-split` as an instance of `split`. */
export const rightSplitViaSplit = split(beside, below);

/** The book's `up-split` as an instance of `split`. */
export const upSplitViaSplit = split(below, beside);
