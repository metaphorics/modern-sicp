// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { cons, type List, map, nil, square } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.21: square-list in the book's two spellings — the explicit
 * cons recursion, and the same process delegated to the section's map.
 */

/** The explicit spelling: square of the head consed onto the mapped tail. */
export function squareListDirect(items: List<number>): List<number> {
  return items._tag === "Nil" ? nil : cons(square(items.head), squareListDirect(items.tail));
}

/** The map spelling: square lifted over the list. */
export function squareListViaMap(items: List<number>): List<number> {
  return map(square, items);
}
