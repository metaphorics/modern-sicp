// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate, enumerateTree, map } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.35: count-leaves as an accumulation. The book maps
 * enumerate-tree over the tree, turning every leaf into a one-element
 * fringe, and accumulates lengths. This edition already exports
 * enumerate-tree as a sequence operation returning the whole fringe at
 * once, so the adaptation maps the constant one over that fringe and
 * lets accumulate sum the ones.
 */

/** Counts the leaves of `t` as an accumulation over its fringe. */
export const countLeavesViaAccumulate = (t: Tree<number>): number =>
  accumulate(
    (a, b) => a + b,
    0,
    map(() => 1, enumerateTree(t)),
  );
