// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.17: define last-pair, which returns the list that contains
 * only the last element of a nonempty list. It cdrs down until it
 * reaches the cell whose tail is the empty list and returns that cell.
 */

/** The last pair of a nonempty list: the cell whose tail is empty. */
export function lastPair(items: List<number>): List<number> {
  if (items._tag === "Cons" && items.tail._tag === "Cons") {
    return lastPair(items.tail);
  }
  return items;
}
