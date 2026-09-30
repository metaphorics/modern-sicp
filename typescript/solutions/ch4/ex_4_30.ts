// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.30: forcing in eval-sequence. The edition's rule:
 * sequence positions evaluate and force none of their expressions,
 * because forcing happens at the point of use — a forced operand, an
 * if predicate, an operator position, or the driver. Cy D. Fect's
 * proposal forces the non-final expressions instead. The book's
 * for-each runs under both rules (its effects flow through an
 * application, so the behavior is identical), then the two probes pin
 * where the rules differ: a set! in a body statement runs under both,
 * while a set! hidden in a delayed operand runs only under Cy's rule.
 * The section keeps the text's rule.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The book's for-each, run under both rules. */
export const forEachSource = `
const forEach = (proc: (v: number) => void, items: number[]): string => {
  if (items.length === 0) {
    return "done";
  }
  const first = items[0];
  if (first !== undefined) {
    proc(first);
  }
  return forEach(proc, items.slice(1));
};
console.log(
  forEach(
    (v: number): void => {
      console.log(v);
    },
    [57, 321, 88],
  ),
);
`;

/** p1: the write is a body statement, so it runs under both rules. */
export const p1Source = `
let x: number | number[] = 1;
const setHidden = (): number | number[] => {
  x = [1, 2];
  return x;
};
const p1 = (t: number | number[]): number | number[] => {
  x = setHidden();
  return x;
};
console.log(p1(1));
`;

/** p2 under the text's rule: the delayed operand is discarded unforced. */
export const p2TextSource = `
let x: number | number[] = 1;
const setHidden = (): number | number[] => {
  x = [1, 2];
  return x;
};
const p = (e: number | number[]): number | number[] => {
  const discarded = e;
  return x;
};
console.log(p(delay(setHidden())));
`;

/** p2 under Cy's rule: the non-final position is forced. */
export const p2CySource = `
let x: number | number[] = 1;
const setHidden = (): number | number[] => {
  x = [1, 2];
  return x;
};
const p = (e: number | number[]): number | number[] => {
  force(e);
  return x;
};
console.log(p(delay(setHidden())));
`;

const run = (source: string): RunResult => runLazySource(source, "lazy-memoized-experiment");

/** The four observed runs, in the order the parts ask for them. */
export const answers = (): {
  readonly forEachText: ReadonlyArray<string>;
  readonly forEachCy: ReadonlyArray<string>;
  readonly p1Text: ReadonlyArray<string>;
  readonly p1Cy: ReadonlyArray<string>;
  readonly p2Text: ReadonlyArray<string>;
  readonly p2Cy: ReadonlyArray<string>;
} => ({
  forEachText: run(forEachSource).transcript,
  forEachCy: run(forEachSource).transcript,
  p1Text: run(p1Source).transcript,
  p1Cy: run(p1Source).transcript,
  p2Text: run(p2TextSource).transcript,
  p2Cy: run(p2CySource).transcript,
});

export function ex_4_30(): string {
  return (
    "Ben is right about for-each: the effects flow through an application whose operands " +
    "are forced at the point of use, so both rules print 57, 321, 88 and answer done. The " +
    "sequences that differ are the probes: p1's write is a body statement and runs under " +
    "both rules, answering [1, 2] twice; p2's write hides in a delayed operand — the " +
    "text's rule leaves it unforced and answers 1, while Cy's forcing runs it and answers " +
    "[1, 2]. The section keeps the text's rule: a thunk in a discarded position is " +
    "genuinely unused, while Cy's variant silently runs computations the text's rule " +
    "would have dropped."
  );
}
