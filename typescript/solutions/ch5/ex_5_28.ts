// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  type ControllerLine,
  jump,
  jumpReg,
  lbl,
  mark,
  op,
  reg,
  restore,
  save,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  evaluatorController,
  replaceSegment,
  runMonitoredEvaluator,
} from "../../packages/ch5/src/04-eceval.js";
import type { StackRow } from "./ex_5_26.js";

// The naive ev-sequence of the 5.4.2 footnote: every expression of a
// sequence, the last one included, goes through the save/restore
// cycle, so no expression is in tail position and a tail call pushes.
const naiveEvSequence: ControllerLine[] = [
  test("no-more-exps?", reg("unev")),
  branch("ev-sequence-end"),
  assign("exp", op("first-exp", reg("unev"))),
  save("unev"),
  save("env"),
  assign("continue", lbl("ev-sequence-continue")),
  jump("eval-dispatch"),
  mark("ev-sequence-continue"),
  restore("env"),
  restore("unev"),
  assign("unev", op("rest-exps", reg("unev"))),
  jump("ev-sequence"),
  mark("ev-sequence-end"),
  restore("continue"),
  jumpReg("continue"),
];

export const naiveEvSequenceController: readonly ControllerLine[] = replaceSegment(
  evaluatorController,
  "ev-sequence",
  "ev-if",
  naiveEvSequence,
);

const RECURSIVE_FACTORIAL = `
(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))
`;

const ITERATIVE_FACTORIAL = `
(define (factorial n)
  (define (iter product counter)
    (if (> counter n)
        product
        (iter (* counter product)
              (+ counter 1))))
  (iter 1 1))
`;

// The 5.26 and 5.27 experiments rerun on the evaluator whose tail
// recursion has been removed. The measured constant-space depth of the
// iterative version now grows linearly, the exercise's demonstration.
export const naiveRecursiveFactorialStack = (n: number): StackRow => {
  const run = runMonitoredEvaluator(
    `${RECURSIVE_FACTORIAL}
(factorial ${n})`,
    naiveEvSequenceController,
  );
  return { n, pushes: run.pushes, maximumDepth: run.maximumDepth };
};

export const naiveIterativeFactorialStack = (n: number): StackRow => {
  const run = runMonitoredEvaluator(
    `${ITERATIVE_FACTORIAL}
(factorial ${n})`,
    naiveEvSequenceController,
  );
  return { n, pushes: run.pushes, maximumDepth: run.maximumDepth };
};

export const naiveFactorialValue = (n: number): string => {
  const run = runMonitoredEvaluator(
    `${RECURSIVE_FACTORIAL}
(factorial ${n})`,
    naiveEvSequenceController,
  );
  return run.transcript.at(-2) as string;
};

export const ex_5_28 = (): readonly StackRow[] => [
  ...[1, 2, 3, 4, 5].map(naiveRecursiveFactorialStack),
  ...[1, 2, 3, 4, 5].map(naiveIterativeFactorialStack),
];
