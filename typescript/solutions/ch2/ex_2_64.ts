// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  car,
  cdr,
  getOrElse,
  type List,
  length,
  nil,
} from "../../packages/ch2/src/02-picture-language.js";
import {
  emptyTreeSet,
  makeTreeSet,
  type TreeSet,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.64: the book's list->tree. The book's `partial-tree`
 * returns a pair whose car is the tree and whose cdr is the unused
 * elements; this edition's tuple `[TreeSet, List<number>]` carries the
 * two halves. The left subtree gets floor((n-1)/2) elements, the entry
 * comes next, and the right subtree takes what remains, so every node
 * splits its range almost exactly in half.
 */
export const partialTree = (elts: List<number>, n: number): [TreeSet, List<number>] => {
  if (n === 0) {
    return [emptyTreeSet, elts];
  }
  const leftSize = Math.floor((n - 1) / 2);
  const [leftTree, nonLeftElts] = partialTree(elts, leftSize);
  const rightSize = n - (leftSize + 1);
  const thisEntry = getOrElse(car(nonLeftElts), 0);
  const [rightTree, remainingElts] = partialTree(getOrElse(cdr(nonLeftElts), nil), rightSize);
  return [makeTreeSet(thisEntry, leftTree, rightTree), remainingElts];
};

/** The book's `list->tree`: balances the whole ordered list. */
export const listToTree = (elements: List<number>): TreeSet =>
  partialTree(elements, length(elements))[0] ?? emptyTreeSet;
