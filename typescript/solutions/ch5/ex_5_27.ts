// SPDX-License-Identifier: GPL-3.0-only
import type { StackRow } from "./ex_5_26.js";
export const recursiveFactorialStack = (n: number): StackRow => ({
  n,
  pushes: 32 * n - 16,
  maximumDepth: 3 * n + 14,
});
export const ex_5_27 = recursiveFactorialStack;
