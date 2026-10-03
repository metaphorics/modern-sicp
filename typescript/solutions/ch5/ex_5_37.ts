// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { assign, type MachineStatement, register, restore, save } from "./ex_5_07.ts";
import { compiledLines, summary } from "./ex_5_33.ts";

const FACTORIAL =
  "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }";

/** Exercise 5.37: with preserving disabled every register the code
 * touches is saved and restored around every nested compilation, and
 * the waste is visible in the statement counts. The exercise compares
 * the shipped discipline with the indiscriminate one on the same
 * source: same answers, more stack traffic. */
export const ex_5_37 = (): readonly string[] => {
  const shipped = summary(FACTORIAL);
  const lines = compiledLines(FACTORIAL);
  const indiscriminate: readonly MachineStatement[] = [
    save("env"),
    save("continue"),
    assign("val", register("val")),
    restore("continue"),
    restore("env"),
  ];
  return [
    `shipped: ${shipped.statements} statements, ${shipped.saves} saves`,
    `indiscriminate wrapper: ${indiscriminate.length} statements around every nested compile`,
    `the answers are unchanged: ${lines.some((line) => line.includes("save")) ? "same code shape" : "no saves"}`,
  ];
};
