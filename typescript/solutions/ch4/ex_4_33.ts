// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.33: list literals produce lazy lists. Ben's error is
 * real: the lazy pair procedures and the strict list data live in
 * different representations, so the procedural `car` over strict data
 * faults. The fix lifts the ordinary list literal into the lazy cons
 * chain of its elements — every element delayed, evaluated in the
 * current environment — so the list the driver hands out is the same
 * lazy structure the program builds by hand; atoms and the empty state
 * stay ordinary data.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The lazy list structure and the lifted literal. */
export const lazyListSource = `
type LazyItem = number | null | { head: LazyItem; tail: LazyItem };
const consL = (h: LazyItem, t: LazyItem): LazyItem => ({ head: h, tail: t });
const carL = (p: LazyItem): LazyItem => {
  if (typeof p === "number" || p === null) {
    return force((p === null ? 0 : p));
  }
  return force(p.head);
};
const cdrL = (p: LazyItem): LazyItem => {
  if (typeof p === "number" || p === null) {
    return force((p === null ? 0 : p));
  }
  return force(p.tail);
};
const lift = (items: number[], i: number): LazyItem => {
  if (i >= items.length) {
    return null;
  }
  const head = items[i];
  return head === undefined ? null : consL(delay(head), delay(lift(items, i + 1)));
};
const listRef = (p: LazyItem, n: number): LazyItem => (n === 0 ? carL(p) : listRef(cdrL(p), n - 1));
`;

/** The lifted literal answers through the lazy procedures. */
export const liftedSource = `${lazyListSource}
console.log(carL(lift([1, 2, 3], 0)));
console.log(carL(cdrL(lift([1, 2, 3], 0))));
console.log(listRef(lift([1, 2, 3, 4], 0), 3));
`;

/** The plain structure through the lazy procedures fails. */
export const plainSource = `${lazyListSource}
console.log(carL(0));
`;

const run = (source: string): RunResult => runLazySource(source, "lazy-memoized-experiment");

/** The observed runs: the lifted answers and the plain failure. */
export const answers = (): {
  readonly lifted: ReadonlyArray<string>;
  readonly plainOutcome: string;
} => ({
  lifted: run(liftedSource).transcript,
  plainOutcome: run(plainSource).outcome.tag,
});

export function ex_4_33(): string {
  return (
    "Lifting the literal into the lazy cons chain makes the list the lazy procedures " +
    "expect: car answers 1, car of cdr answers 2, and listRef walks [1, 2, 3, 4] to 4 — " +
    "the a, b, d of the book's example. The plain structure through the lazy procedures " +
    "fails: an atom is not a lazy pair, and forcing it answers the typed bad-operand " +
    'fault where the old car answered "not a procedure". Atoms and the empty state stay ' +
    "ordinary data."
  );
}
