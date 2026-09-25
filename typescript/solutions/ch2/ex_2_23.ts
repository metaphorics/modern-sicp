// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.23: for-each is map's cousin that uses the results for
 * something else, like printing. It applies `f` to each element left to
 * right and, per the book's "something arbitrary, such as true",
 * returns true.
 */

/** Applies `f` to every element left to right; returns true. */
export function forEach<A>(f: (x: A) => void, items: List<A>): boolean {
  let rest = items;
  while (rest._tag === "Cons") {
    f(rest.head);
    rest = rest.tail;
  }
  return true;
}
