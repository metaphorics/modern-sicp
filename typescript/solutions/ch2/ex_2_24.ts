// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  countLeaves,
  type List,
  leaf,
  length,
  list,
  node,
  type Showable,
  showList,
  type Tree,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.24: the structure of `[1, [2, [3, 4]]]`. The
 * edition builds the value and reads its shape back from the printers:
 * the printed form, the outer length, and the tree interpretation's
 * leaf count.
 */

const value: List<Showable> = list<Showable>(1, list<Showable>(2, list<Showable>(3, 4)));

/** How the value prints. */
export function printedForm(): string {
  return showList(value);
}

/** The outer list holds two elements, the second itself a list. */
export function listLength(): number {
  return length(value);
}

/** The tree reading has one leaf per number. */
export function leafCount(): number {
  const tree: Tree<number> = node(leaf(1), node(leaf(2), node(leaf(3), leaf(4))));
  return countLeaves(tree);
}
