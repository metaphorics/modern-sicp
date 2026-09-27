// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { isMNil, type MList } from "../../packages/ch3/src/03-mutable-data.js";
import { isMListValue } from "./ex_3_16.js";

/**
 * Exercise 3.17a, an edition addition on the 3.17 row: count the
 * distinct nodes of a shared structure, not just its pairs. A
 * box-and-pointer diagram also draws the atoms, so the node count is
 * the distinct pairs, by identity, plus the distinct leaf atoms, by
 * value.
 */

/** The number of distinct nodes in a structure: mutable pairs are
 * deduplicated by identity (a host `Set<object>`), leaf atoms by
 * value (a host value `Set`); the walk covers head and tail from the
 * root. */
export const countNodes = (x: MList<unknown>): number => {
  const pairs = new Set<object>();
  const atoms = new Set<unknown>();
  const countAtom = (value: unknown): number => {
    if (atoms.has(value)) {
      return 0;
    }
    atoms.add(value);
    return 1;
  };
  const walk = (node: MList<unknown>): number => {
    if (isMNil(node) || pairs.has(node)) {
      return 0;
    }
    pairs.add(node);
    const headCount = isMListValue(node.head) ? walk(node.head) : countAtom(node.head);
    return 1 + headCount + walk(node.tail);
  };
  return walk(x);
};
