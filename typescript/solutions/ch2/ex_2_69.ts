// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import type { List } from "../../packages/ch2/src/02-picture-language.js";
import {
  adjoinSetWeighted,
  type HuffTree,
  makeCodeTree,
  makeLeafSet,
  type SymbolFrequency,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.69: `successiveMerge` repeatedly merges the two
 * smallest-weight elements of the ordered set with `makeCodeTree` and
 * reinserts the merged node by weight, until one tree remains. The
 * ordered set does all the work: the two smallest elements are always
 * the first two.
 */
export const successiveMerge = (set: List<HuffTree>): Result<HuffTree, string> => {
  if (set._tag === "Nil") {
    return err("successive-merge of an empty set");
  }
  if (set.tail._tag === "Nil") {
    return ok(set.head);
  }
  const first = set.head;
  const rest = set.tail;
  const merged = makeCodeTree(first, rest.head);
  return successiveMerge(adjoinSetWeighted(merged, rest.tail));
};

export const generateHuffmanTree = (pairs: List<SymbolFrequency>): Result<HuffTree, string> =>
  successiveMerge(makeLeafSet(pairs));
