// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { runMonitoredEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { ITERATIVE_FACTORIAL, TREE_FIB } from "./ex_5_26.ts";
import { summary } from "./ex_5_33.ts";

/** One comparison row: the interpreted machine's own stack counters
 * beside the compiled code's save traffic for the same computation. */
export type RatioRow = {
  readonly pushes: number;
  readonly maxDepth: number;
  readonly compiledSaves: number;
};

const measure = (definition: string, call: string, compiledSource: string): RatioRow => {
  const run = runMonitoredEvaluator(`${definition}\n${call};`);
  return {
    pushes: run.stackStats.pushes,
    maxDepth: run.stackStats.maxDepth,
    compiledSaves: summary(compiledSource).saves,
  };
};

/** Exercise 5.45: the stack ratios of compiled versus interpreted
 * factorial. Both numbers come from real runs: the interpreter's
 * counters from the monitored machine, the compiled traffic from the
 * compiled statements themselves. */
export const ex_5_45 = (): readonly string[] => {
  const rows = [3, 4, 5, 6].map((n) => ({
    n,
    ...measure(
      ITERATIVE_FACTORIAL,
      `factorial(${n})`,
      "function factorial(n: number): number { function iter(p: number, c: number): number { return c > n ? p : iter(c * p, c + 1); } return iter(1, 1); }",
    ),
  }));
  return rows.map(
    (row) =>
      `n = ${row.n}: interpreted pushes ${row.pushes}, depth ${row.maxDepth}, compiled saves ${row.compiledSaves}`,
  );
};

/** Exercise 5.46: the same comparison for the tree-recursive Fibonacci,
 * where the compiled save traffic follows the recursion tree. */
export const ex_5_46 = (): readonly string[] => {
  const rows = [3, 5, 7].map((n) => ({
    n,
    ...measure(
      TREE_FIB,
      `fib(${n})`,
      "function fib(n: number): number { return n < 2 ? n : fib(n - 1) + fib(n - 2); }",
    ),
  }));
  return rows.map(
    (row) =>
      `n = ${row.n}: interpreted pushes ${row.pushes}, depth ${row.maxDepth}, compiled saves ${row.compiledSaves}`,
  );
};
