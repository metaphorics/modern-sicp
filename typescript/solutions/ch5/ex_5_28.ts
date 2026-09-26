// SPDX-License-Identifier: GPL-3.0-only
import type { StackRow } from "./ex_5_26.js";
export const tailRecursiveFactorialStack = (n: number): StackRow => ({
  n,
  pushes: 8 * n + 3,
  maximumDepth: 3 * n + 14,
});
export const ex_5_28 = tailRecursiveFactorialStack;
