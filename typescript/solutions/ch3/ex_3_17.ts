// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { isMNil, type MList } from "../../packages/ch3/src/03-mutable-data.js";
import { isMListValue } from "./ex_3_16.js";

/**
 * Exercise 3.17: a correct count-pairs. The book's hint: traverse the
 * structure with an auxiliary record of the pairs already counted.
 * The edition uses a host `Set<object>`, whose membership test is
 * object identity, the book's `eq?`. Walking head and tail then
 * counts every distinct pair exactly once and terminates on rings.
 */

/** The number of distinct pairs in any structure, however shared or
 * cyclic: walk head and tail, and count a pair only the first time
 * the identity set sees it. */
export const countPairsCorrect = (x: MList<unknown>): number => {
  const seen = new Set<object>();
  const walk = (node: MList<unknown>): number => {
    if (isMNil(node) || seen.has(node)) {
      return 0;
    }
    seen.add(node);
    return 1 + (isMListValue(node.head) ? walk(node.head) : 0) + walk(node.tail);
  };
  return walk(x);
};
