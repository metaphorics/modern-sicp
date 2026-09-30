// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { formatMachineStatement } from "../../packages/ch5/src/01-register-machines.ts";
import { readProgram } from "../../packages/ch5/src/04-eceval.ts";
import { compileProgram, isCompileError } from "../../packages/ch5/src/05-compilation.ts";

/** Renders one compiled program the way the book prints its figures:
 * one controller line per statement, labels bare. */
export const compiledLines = (source: string): readonly string[] => {
  const compiled = compileProgram(readProgram(source));
  if (isCompileError(compiled)) {
    throw new Error(`compilation failed: ${JSON.stringify(compiled)}`);
  }
  return compiled.instructions.map((statement) => formatMachineStatement(statement));
};

/** The counts one compilation gathers: statements, saves, restores. */
export const summary = (
  source: string,
): { statements: number; saves: number; restores: number } => {
  const lines = compiledLines(source);
  return {
    statements: lines.length,
    saves: lines.filter((line) => line.startsWith("save ")).length,
    restores: lines.filter((line) => line.startsWith("restore ")).length,
  };
};

const ALT =
  "function factorialAlt(n: number): number { return n === 1 ? 1 : n * factorialAlt(n - 1); }";
const BOOK = "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }";

/** Exercise 5.33: the two orderings of the multiplication compile to
 * the same size of code with the same save discipline; the operand
 * order, not the code shape, is what differs. */
export const ex_5_33 = (): readonly string[] => {
  const alt = summary(ALT);
  const book = summary(BOOK);
  return [
    `factorial-alt: ${alt.statements} statements, ${alt.saves} saves`,
    `factorial: ${book.statements} statements, ${book.saves} saves`,
    "the difference is the multiplication's operand order, not the code shape",
  ];
};
