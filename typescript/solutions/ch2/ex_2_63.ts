// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { append, cons, type List, nil } from "../../packages/ch2/src/02-picture-language.js";
import type { TreeSet } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.63: the book's two tree->list procedures over the
 * section's tree union. `treeToList1` recurses on both subtrees and
 * appends; `treeToList2` threads an accumulator through
 * `copyToList`. Both produce the in-order list, so both answer part
 * (a) the same for every tree; the growth difference is in the
 * rationale.
 */
export const treeToList1 = (tree: TreeSet): List<number> =>
  tree._tag === "Empty"
    ? nil
    : append(treeToList1(tree.left), cons(tree.entry, treeToList1(tree.right)));

const copyToList = (tree: TreeSet, resultList: List<number>): List<number> =>
  tree._tag === "Empty"
    ? resultList
    : copyToList(tree.left, cons(tree.entry, copyToList(tree.right, resultList)));

export const treeToList2 = (tree: TreeSet): List<number> => copyToList(tree, nil);
