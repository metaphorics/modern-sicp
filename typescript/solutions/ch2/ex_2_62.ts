// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { cons, type List } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.62: `unionSetOrdered` for the ordered representation.
 * Advance whichever head is smaller, consuming both sets in one pass:
 * Theta(n1 + n2) steps rather than the unordered Theta(n1 * n2).
 */
export const unionSetOrdered = (set1: List<number>, set2: List<number>): List<number> => {
  if (set1._tag === "Nil") {
    return set2;
  }
  if (set2._tag === "Nil") {
    return set1;
  }
  const x1 = set1.head;
  const x2 = set2.head;
  if (x1 === x2) {
    return cons(x1, unionSetOrdered(set1.tail, set2.tail));
  }
  return x1 < x2
    ? cons(x1, unionSetOrdered(set1.tail, set2))
    : cons(x2, unionSetOrdered(set1, set2.tail));
};
