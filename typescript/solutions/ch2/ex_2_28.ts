// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  append,
  type List,
  list,
  nil,
  type Tree,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.28: fringe returns the leaves of a tree left to right as a
 * list. A leaf contributes its one-element list; a node appends the
 * fringe of each subtree in order with the section's `append`.
 */

/** The leaves of `t`, left to right. */
export function fringe(t: Tree<number>): List<number> {
  if (t._tag === "Leaf") {
    return list(t.value);
  }
  return t.subtrees.reduce<List<number>>((acc, sub) => append(acc, fringe(sub)), nil);
}
