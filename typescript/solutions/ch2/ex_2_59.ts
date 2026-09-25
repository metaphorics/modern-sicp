// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { cons, type List } from "../../packages/ch2/src/02-picture-language.js";
import { elementOfSetUnordered } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.59: `unionSetUnordered` for the unordered-list
 * representation. Keep the elements of `set1` that are new to `set2`
 * in front of `set2`; like the book's other unordered operations it is
 * Theta(n^2) for two sets of size n, since every membership test can
 * scan all of `set2`.
 */
export const unionSetUnordered = (set1: List<number>, set2: List<number>): List<number> =>
  set1._tag === "Nil"
    ? set2
    : elementOfSetUnordered(set1.head, set2)
      ? unionSetUnordered(set1.tail, set2)
      : cons(set1.head, unionSetUnordered(set1.tail, set2));
