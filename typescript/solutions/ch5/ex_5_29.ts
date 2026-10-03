// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { stackTable, TREE_FIB } from "./ex_5_26.ts";

/** Exercise 5.29: the tree-recursive Fibonacci machine's stack. The
 * maximum depth is set by the longest chain of deferred operations,
 * which grows linearly with n; the total pushes grow with the tree
 * itself. */
export const ex_5_29 = (): readonly { n: number; pushes: number; maxDepth: number }[] =>
  stackTable(TREE_FIB, (n) => `fib(${n})`, [2, 3, 4, 5, 6]);
