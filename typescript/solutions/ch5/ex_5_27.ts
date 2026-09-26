// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { runMonitoredEvaluator } from "../../packages/ch5/src/04-eceval.js";
import type { StackRow } from "./ex_5_26.js";

// The recursive factorial of exercise 5.27.
const RECURSIVE_FACTORIAL = `
(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))
`;

// One monitored session: define the procedure, call it, and read the
// stack counters of that final interaction. For n = 5 this reproduces
// the book's printed session line, 144 pushes at depth 28.
export const recursiveFactorialStack = (n: number): StackRow => {
  const run = runMonitoredEvaluator(`${RECURSIVE_FACTORIAL}
(factorial ${n})`);
  return { n, pushes: run.pushes, maximumDepth: run.maximumDepth };
};

export const recursiveFactorialTable = (): readonly StackRow[] =>
  [1, 2, 3, 4, 5, 6].map(recursiveFactorialStack);

export const ex_5_27 = recursiveFactorialTable;
