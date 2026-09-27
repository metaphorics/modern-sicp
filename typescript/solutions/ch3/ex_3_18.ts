// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.18: whether a list contains a cycle, that is, whether a
 * program that tried to find the end by taking successive cdrs would
 * go into an infinite loop. Exercise 3.13 constructed such lists with
 * a set-cdr! back to an earlier pair. The book's answer walks the
 * tails while remembering every pair it has seen; JavaScript object
 * identity does the book's `eq?`, and the host `Set` is the
 * identity-keyed collection the exercise-map row calls for (the row
 * marks TypeScript T where the map's other editions read A for
 * lacking one). A tail that revisits any remembered pair proves the
 * cycle before any loop could run away.
 */

/** Whether following the tails of `x` from pair to pair ever revisits
 * a pair: the book's `has-cycle?`. The empty list, and any list whose
 * tail chain eventually reaches it, answer false; a list whose last
 * tail points back at an earlier pair answers true at the first
 * revisit. The book's cdr-chase is the `tail` step here. */
export const hasCycle = (x: MList<unknown>): boolean => {
  const visited = new Set<object>();
  let rest = x;
  while (rest._tag === "MCons") {
    if (visited.has(rest)) {
      return true;
    }
    visited.add(rest);
    rest = rest.tail;
  }
  return false;
};
