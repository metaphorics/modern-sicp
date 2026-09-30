// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.53: permanent accumulation under failure. Every prime-sum
 * pair is permanently consed onto the list, then the branch fails so
 * the nearest choice tries another pair. When the choices are spent,
 * `ifFail` catches the primary failure and returns the accumulated list
 * once. The answer therefore reverses discovery order; all effects and
 * fallback delivery happen in the guest evaluator.
 */
import { runAmbAnswers, type SearchRun } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** The book's prime-sum search, permanent accumulation, and ifFail. */
export const pairsSource = `
const remainder = (a: number, b: number): number => a % b;
const smallest = (n: number, d: number): number =>
  (d * d > n ? n : (remainder(n, d) === 0 ? d : smallest(n, d + 1)));
const isPrime = (n: number): boolean => n > 1 && smallest(n, 2) === n;
const anElementOf = (items: number[]): number => {
  require(items.length > 0);
  const first = items[0];
  return choose(first === undefined ? -1 : first, anElementOf(items.slice(1)));
};
let pairs: number[][] = [];
const findAndAccumulate = (): number[] => {
  const a = anElementOf([1, 3, 5, 8]);
  const b = anElementOf([20, 35, 110]);
  require(isPrime(a + b));
  const pair = [a, b];
  permanentAssign(pairs, [pair, ...pairs]);
  require(false);
  return pair;
};
ifFail(findAndAccumulate(), pairs);
`;

/** The actual search run, with live failure and step counters. */
export const pairRun = (): SearchRun => runAmbAnswers(pairsSource, "amb-depth-first-experiment", 1);

/** The handler's accumulated list, rendered. */
export const accumulatedPairs = (): ReadonlyArray<string> =>
  pairRun().answers.map((value) => format(value));

export function ex_4_53(): string {
  const run = pairRun();
  return (
    `The guest handler delivers ${run.answers.map((value) => format(value)).join("")}; ` +
    `the search records ${run.failures} failed computations and ${run.steps} ` +
    `deferred steps before exhaustion. The permanent writes survive each ` +
    `failed branch; ifFail runs only after the combinations are spent.`
  );
}
