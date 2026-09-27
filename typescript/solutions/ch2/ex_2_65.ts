// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { intersectionSetOrdered, type TreeSet } from "../../packages/ch2/src/03-symbolic-data.js";
import { unionSetOrdered } from "./ex_2_62.js";
import { treeToList2 } from "./ex_2_63.js";
import { listToTree, partialTree } from "./ex_2_64.js";

/**
 * Exercise 2.65: Theta(n) union and intersection for tree sets.
 * Convert each tree to its ordered list (Theta(n)), run the ordered
 * list operation (Theta(n1 + n2)), and rebalance with the book's
 * `listToTree` (Theta(n)). The ordered-list union is exercise 2.62's;
 * the ordered-list intersection is the section's.
 */

export const unionSetTree = (t1: TreeSet, t2: TreeSet): TreeSet =>
  listToTree(unionSetOrdered(treeToList2(t1), treeToList2(t2)));

export const intersectionSetTree = (t1: TreeSet, t2: TreeSet): TreeSet =>
  listToTree(intersectionSetOrdered(treeToList2(t1), treeToList2(t2)));

/** Exposed for the test: the ordered spine a tree unwinds to. */
export const treeElements = (t: TreeSet): List<number> => treeToList2(t);

export { partialTree };
