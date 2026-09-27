// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";
import { leaf, node } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.31: abstract the tree recursion of exercise 2.30 into
 * `treeMap`, which applies any function to every leaf. Squaring is then
 * one application, the same lift that relates the module's scaleTree to
 * its scaleTreeViaMap: only the function at the leaves changes.
 */

/** Applies `f` to every leaf of `t`, preserving the shape. */
export const treeMap = (f: (x: number) => number, t: Tree<number>): Tree<number> =>
  t._tag === "Leaf" ? leaf(f(t.value)) : node(...t.subtrees.map((sub) => treeMap(f, sub)));

/** Exercise 2.30's squaring, recovered as a one-line treeMap application. */
export const squareTree = (t: Tree<number>): Tree<number> => treeMap((x) => x * x, t);
