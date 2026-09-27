// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { cons, type List } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.61: `adjoinSetOrdered` for the ordered representation.
 * Walk until the insertion point: stop with the set unchanged on an
 * exact hit, stop when the head passes `x`, otherwise rebuild the spine
 * and cons `x` in place. Like the book's ordered membership test, this
 * examines on the average about half the elements instead of all of
 * them.
 */
export const adjoinSetOrdered = (x: number, set: List<number>): List<number> => {
  if (set._tag === "Nil" || x < set.head) {
    return cons(x, set);
  }
  if (x === set.head) {
    return set;
  }
  return cons(set.head, adjoinSetOrdered(x, set.tail));
};
