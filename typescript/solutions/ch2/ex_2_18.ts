// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { append, type List, list, nil } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.18: reverse takes a list and returns a list of the same
 * elements in reverse order. This is the recursive append-style
 * spelling the book sketches first: the reverse of a list is the
 * reverse of its tail with the head moved to the end.
 */

/** The elements of `items` in reverse order. */
export function reverse(items: List<number>): List<number> {
  return items._tag === "Nil" ? nil : append(reverse(items.tail), list(items.head));
}
