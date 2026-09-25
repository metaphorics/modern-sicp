// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { node, type Tree } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.27: deep-reverse for the tree union. The book's x, ((1 2)
 * (3 4)), is a node of two nodes here. Deep reversal reverses every
 * node's subtree order; shallow reversal, the contrast case, flips only
 * the outermost node's and leaves the children untouched. Both copy the
 * subtrees with a spread before reversing because the array is
 * readonly.
 */

/** Reverses the subtree order at every level. */
export function deepReverse(t: Tree<number>): Tree<number> {
  if (t._tag === "Leaf") {
    return t;
  }
  return node(...[...t.subtrees].reverse().map((sub) => deepReverse(sub)));
}

/** Reverses only the outermost node's subtree order. */
export function shallowReverse(t: Tree<number>): Tree<number> {
  if (t._tag === "Leaf") {
    return t;
  }
  return node(...[...t.subtrees].reverse());
}
