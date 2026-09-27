// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";
import { leaf, node } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.30: square every leaf of a tree, preserving its shape. The
 * direct spelling recurses over the two constructors; the `map` spelling
 * is the book's alternative, mapping a squaring function over the
 * subtrees and deciding leaf against branch inside the mapping function.
 */

/** Squares each leaf by explicit recursion over Leaf and Node. */
export const squareTreeDirect = (t: Tree<number>): Tree<number> =>
  t._tag === "Leaf" ? leaf(t.value * t.value) : node(...t.subtrees.map(squareTreeDirect));

/** Squares each leaf by mapping over the subtrees, leaf case inside. */
export const squareTreeViaMap = (t: Tree<number>): Tree<number> =>
  t._tag === "Leaf"
    ? leaf(t.value * t.value)
    : node(
        ...t.subtrees.map((sub) =>
          sub._tag === "Leaf" ? leaf(sub.value * sub.value) : squareTreeViaMap(sub),
        ),
      );
