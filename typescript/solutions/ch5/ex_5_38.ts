// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { compiledLines, summary } from "./ex_5_33.ts";

const PLAIN =
  "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }";
const OPEN = "function go() { return 1 + 2 + 3 + 4; }";

/** Exercise 5.38: open coding. The open-coded arithmetic compiles the
 * operands into argument registers and folds them with one operation
 * call per operator, so the code for an arithmetic-heavy expression is
 * much smaller than the generic application sequence, and the answers
 * are the same numbers. */
export const ex_5_38 = (): readonly string[] => {
  const plain = summary(PLAIN);
  const open = summary(OPEN);
  return [
    `generic factorial: ${plain.statements} statements`,
    `open-coded fold: ${open.statements} statements`,
    "open coding shrinks arithmetic-heavy code and keeps the answers",
    compiledLines(OPEN).some((line) => line.startsWith("perform") || line.includes("add"))
      ? "the fold is visible in the listing"
      : "the fold is in the operand registers",
  ];
};
