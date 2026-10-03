// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { compiledLines } from "./ex_5_33.ts";

/** Exercise 5.35: the expression behind the book's compiled figure,
 * compiled and rendered statement for statement. The labels are the
 * compiler's own fresh names; the exercise is the reverse-engineering
 * of the source from the compiled shape. */
export const ex_5_35 = (): readonly string[] => {
  const source = "function f(x: number) { return x + g(x + 2); }";
  return [`expression: ${source}`, "figure reproduced:", ...compiledLines(source)];
};
