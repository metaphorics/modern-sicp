// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
import { runSource } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.32: streams versus lazy lists. The procedural pair delays
 * both slots, so a pair constructs without computing and a dormant
 * armed slot forces only when it is the one demanded. The
 * demonstrations run the armed pair under the lazy experiment and the
 * same procedural cons under the strict base evaluator, which is the
 * eager discipline the chapter 3 comparison rests on. The
 * self-referential `ones` is the constructive demonstration: the pair
 * builds with nothing forced.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The lazy pair demonstrations: demand is what computes. */
export const lazyPairsSource = `
let marks = 0;
const mark = (n: number): number => {
  marks = marks + 1;
  return n;
};
const consL = (h: number, t: number): number[] => [h, t];
const slot = (p: number[], index: number): number => {
  const item = p[index];
  return item === undefined ? 0 : item;
};
const carL = (p: number[]): number => force(slot(p, 0));
const cdrL = (p: number[]): number => force(slot(p, 1));
`;

/** `car` of the armed pair: the head forces, the armed slot never does. */
export const carSource = `${lazyPairsSource}
console.log(carL(consL(delay(mark(7)), delay(mark(force(5))))));
console.log(marks);
`;

/** `cdr` of the armed pair: the division runs only on demand. */
export const cdrSource = `${lazyPairsSource}
console.log(cdrL(consL(delay(mark(7)), delay(mark(1 / 0)))));
console.log(marks);
`;

/** The self-referential `ones`: constructs with nothing forced. */
export const onesSource = `${lazyPairsSource}
let ones: number[] = [];
const build = (): number[] => {
  ones = consL(delay(mark(1)), delay(mark(force(slot(ones, 0)))));
  return ones;
};
console.log(build().length);
console.log(marks);
`;

/** The strict contrast: both slots evaluate at construction. */
export const strictPairsSource = `
let marks = 0;
const mark = (n: number): number => {
  marks = marks + 1;
  return n;
};
const consS = (h: number, t: number): number[] => [h, t];
const carS = (p: number[]): number => {
  const head = p[0];
  return head === undefined ? 0 : head;
};
console.log(carS(consS(mark(1 / 0), mark(7))));
console.log(marks);
`;

const run = (source: string): RunResult => runLazySource(source, "lazy-memoized-experiment");

/** The four observed runs. */
export const answers = (): {
  readonly car: ReadonlyArray<string>;
  readonly cdr: ReadonlyArray<string>;
  readonly ones: ReadonlyArray<string>;
  readonly strict: ReadonlyArray<string>;
} => ({
  car: run(carSource).transcript,
  cdr: run(cdrSource).transcript,
  ones: run(onesSource).transcript,
  strict: runSource(strictPairsSource).transcript,
});

export function ex_4_32(): string {
  return (
    "A delayed pair constructs without computing: car of the armed pair answers 7 with one " +
    "mark, cdr answers the division only on demand with one, and the self-referential " +
    "ones builds with nothing forced. Under the strict base evaluator both slots evaluate " +
    "at construction — the division runs before car is ever entered — which is the eager " +
    "discipline the chapter 3 comparison rests on."
  );
}
