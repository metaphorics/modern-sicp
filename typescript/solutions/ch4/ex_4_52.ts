// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.52: if-fail. The special form catches primary-search
 * exhaustion once: all primary answers arrive first; only when the
 * primary's failure continuation is exhausted does the fallback
 * answer. Subsequent failure backtracks normally. Both book
 * programs run as actual guest ifFail forms and expose live search
 * failure/step counts.
 */
import { runAmbAnswers, type SearchRun } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** Even-finding over a choose-drawn element with recursive remainder. */
export const evenSource = (items: string): string => `
const remainder = (a: number, b: number): number => (a < b ? a : remainder(a - b, b));
const anElementOf = (rest: number[]): number => {
  require(rest.length > 0);
  const first = rest[0];
  return choose(first === undefined ? -1 : first, anElementOf(rest.slice(1)));
};
const evenChoice = (): number => {
  const x = anElementOf([${items}]);
  require(remainder(x, 2) === 0);
  return x;
};
ifFail(evenChoice(), "all-odd");
`;

/** A real ifFail search, including live failure counters. */
export const ifFailRun = (items: string): SearchRun =>
  runAmbAnswers(evenSource(items), "amb-depth-first-experiment", 1);

/** The real search answers for one list, rendered. */
export const ifFailAnswers = (items: string): ReadonlyArray<string> =>
  ifFailRun(items).answers.map((value) => format(value));

export function ex_4_52(): string {
  const odd = ifFailRun("1, 3, 5");
  const withEight = ifFailRun("1, 3, 5, 8");
  return (
    `Without an even element the fallback answers ${odd.answers.map((value) => format(value)).join(", ")}; ` +
    `with 8 the ordered answers are ${withEight.answers.map((value) => format(value)).join(", ")}. ` +
    `The runs record ${odd.failures} and ${withEight.failures} failed computations, ` +
    `and ${odd.steps} and ${withEight.steps} deferred steps. The fallback is delivered ` +
    `once after primary exhaustion. `
  );
}
