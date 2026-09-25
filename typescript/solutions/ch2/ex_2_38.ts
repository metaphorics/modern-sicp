// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.38: fold-left, the mirror of the book's fold-right. Where
 * accumulate (fold-right) combines the head with the fold of the tail,
 * leaving the first element applied last, fold-left walks the list and
 * combines the running result with each element left to right, applying
 * the first element first. The two agree only when `op` is commutative
 * with the initial as a two-sided identity.
 */

/** Folds `sequence` left: `op(op(op(initial, x1), x2), x3)`. */
export const foldLeft = <A, B>(op: (x: B, y: A) => B, initial: B, sequence: List<A>): B => {
  let result = initial;
  let rest = sequence;
  while (rest._tag === "Cons") {
    result = op(result, rest.head);
    rest = rest.tail;
  }
  return result;
};
