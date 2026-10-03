// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { RECURSIVE_FACTORIAL, stackTable } from "./ex_5_26.ts";

/** Exercise 5.27: recursive factorial uses 20 additional pushes and,
 * after the first recursive level, 8 additional stack slots per level. */
export const ex_5_27 = (): readonly { n: number; pushes: number; maxDepth: number }[] =>
  stackTable(RECURSIVE_FACTORIAL, (n) => `factorial(${n})`, [1, 2, 3, 4, 5, 6]);
