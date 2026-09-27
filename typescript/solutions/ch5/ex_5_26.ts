// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { runMonitoredEvaluator } from "../../packages/ch5/src/04-eceval.js";

export interface StackRow {
  readonly n: number;
  readonly pushes: number;
  readonly maximumDepth: number;
}

// The iterative factorial of 1.2.1 that exercise 5.26 asks about.
const ITERATIVE_FACTORIAL = `
(define (factorial n)
  (define (iter product counter)
    (if (> counter n)
        product
        (iter (* counter product)
              (+ counter 1))))
  (iter 1 1))
`;

// One monitored session: define the procedure, then call it, and read
// the stack counters of that final interaction, the way the book's
// 5.4.4 monitored driver captures them.
export const iterativeFactorialStack = (n: number): StackRow => {
  const run = runMonitoredEvaluator(`${ITERATIVE_FACTORIAL}
(factorial ${n})`);
  return { n, pushes: run.pushes, maximumDepth: run.maximumDepth };
};

export const iterativeFactorialTable = (): readonly StackRow[] =>
  [1, 2, 3, 4, 5, 6].map(iterativeFactorialStack);

export const ex_5_26 = iterativeFactorialTable;
