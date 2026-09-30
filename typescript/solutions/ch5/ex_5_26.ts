// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type MeasuredRun, runMonitoredEvaluator } from "../../packages/ch5/src/04-eceval.ts";

/** The iterative factorial of 1.2.1 that exercise 5.26 asks about, in
 * the checked guest source of this edition. */
export const ITERATIVE_FACTORIAL = [
  "function factorial(n: number): number {",
  "  function iter(product: number, counter: number): number {",
  "    return counter > n ? product : iter(counter * product, counter + 1);",
  "  }",
  "  return iter(1, 1);",
  "}",
].join("\n");

/** The recursive factorial of exercise 5.27. */
export const RECURSIVE_FACTORIAL = [
  "function factorial(n: number): number {",
  "  return n === 1 ? 1 : factorial(n - 1) * n;",
  "}",
].join("\n");

/** The tree-recursive Fibonacci of exercise 5.29. */
export const TREE_FIB = [
  "function fib(n: number): number {",
  "  return n < 2 ? n : fib(n - 1) + fib(n - 2);",
  "}",
].join("\n");

/** One monitored session: the definition plus the call in one unit, so
 * the counters cover the whole computation. */
export const measure = (definition: string, call: string): MeasuredRun =>
  runMonitoredEvaluator(`${definition}\n${call};`);

/** The monitored table for one definition over increasing input. */
export const stackTable = (
  definition: string,
  callFor: (n: number) => string,
  inputs: readonly number[],
): readonly { n: number; pushes: number; maxDepth: number }[] =>
  inputs.map((n) => {
    const run = measure(definition, callFor(n));
    return { n, pushes: run.stackStats.pushes, maxDepth: run.stackStats.maxDepth };
  });

/** Exercise 5.26: the iterative process keeps a constant maximum depth
 * however large n grows; the total pushes grow linearly. */
export const ex_5_26 = (): readonly { n: number; pushes: number; maxDepth: number }[] =>
  stackTable(ITERATIVE_FACTORIAL, (n) => `factorial(${n})`, [1, 2, 3, 4, 5, 6]);
