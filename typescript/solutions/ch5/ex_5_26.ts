// SPDX-License-Identifier: GPL-3.0-only
export interface StackRow {
  readonly n: number;
  readonly pushes: number;
  readonly maximumDepth: number;
}
export const iterativeFactorialStack = (n: number): StackRow => ({
  n,
  pushes: 35 * n + 29,
  maximumDepth: 5 * n + 3,
});
export const ex_5_26 = iterativeFactorialStack;
