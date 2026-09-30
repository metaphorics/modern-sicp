// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  evaluatorController,
  gotoLabel,
  type MachineStatement,
  makeEvaluator,
  op,
  register,
  replaceSegment,
  restore,
  runMonitoredEvaluator,
  test,
  type Word,
} from "../../packages/ch5/src/04-eceval.ts";
import { mark } from "./ex_5_07.ts";
import { ITERATIVE_FACTORIAL, RECURSIVE_FACTORIAL } from "./ex_5_26.ts";

type EvaluatorStatement = MachineStatement<Word>;

/** Exercise 5.28: delay return-transfer cleanup until the common
 * application exit path. The de-tailed variant still returns the same
 * values, but every procedure leaves through the ordinary cleanup path;
 * its measured stack depth is never below the base controller's. */
export const deTailedController: readonly EvaluatorStatement[] = replaceSegment(
  evaluatorController,
  "apply-body-done",
  "call-end",
  [
    mark("apply-body-done"),
    restore("env"),
    restore("continue"),
    gotoLabel("apply-body-exit"),
    mark("apply-body-exit"),
    test("isReturnTransfer", register("transfer")),
    branch("apply-body-return-clear"),
    gotoLabel("apply-body-exit-cleanup"),
    mark("apply-body-return-clear"),
    assign("transfer", op("nullTransfer")),
    gotoLabel("apply-body-exit-cleanup"),
    mark("apply-body-exit-cleanup"),
    restore("argl"),
    restore("proc"),
    restore("item"),
    gotoLabel("continue-dispatch"),
  ],
);

const rows = (
  definition: string,
  controller: readonly EvaluatorStatement[],
): readonly {
  n: number;
  pushes: number;
  maxDepth: number;
}[] =>
  [1, 2, 3, 4, 5, 6].map((n) => {
    const run = runMonitoredEvaluator(`${definition}\nfactorial(${n})`, controller);
    return { n, pushes: run.stackStats.pushes, maxDepth: run.stackStats.maxDepth };
  });

const depthStep = (table: readonly { maxDepth: number }[]): number =>
  (table[1]?.maxDepth ?? 0) - (table[0]?.maxDepth ?? 0);

/** The rerun: both processes on both machines, with the answers
 * unchanged and the depth steps beside each other. */
export const ex_5_28 = (): readonly string[] => {
  const baseIterative = rows(ITERATIVE_FACTORIAL, evaluatorController);
  const plainIterative = rows(ITERATIVE_FACTORIAL, deTailedController);
  const baseRecursive = rows(RECURSIVE_FACTORIAL, evaluatorController);
  const plainRecursive = rows(RECURSIVE_FACTORIAL, deTailedController);
  return [
    `iterative, base depth step = ${depthStep(baseIterative)}, de-tailed = ${depthStep(plainIterative)}`,
    `recursive, base depth step = ${depthStep(baseRecursive)}, de-tailed = ${depthStep(plainRecursive)}`,
    `iterative answers unchanged: ${makeEvaluator(`${ITERATIVE_FACTORIAL}\nconsole.log(factorial(5));`, {}, deTailedController).run().transcript.join(" ")}`,
  ];
};
