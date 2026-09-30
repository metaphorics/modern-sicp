// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.31: lazy and lazy-memo parameter declarations. The
 * declaration `(name lazy)` or `(name lazy-memo)` is object-language
 * syntax on the parameter list: a bare name is strict, the
 * upward-compatible default. The guest grammar has no parameter-mode
 * syntax, so the modes are carried by the argument discipline the
 * declaration denotes: a strict parameter arrives as its value, a lazy
 * parameter as a recomputing thunk called per demand, and a lazy-memo
 * parameter as one delayed cell forced per demand under the memoized
 * experiment. The unforced lazy parameter is exactly what lets `pick`
 * return its second arm while the first arm would fault on demand.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The declared-modes session: strict, lazy, and lazy-memo demands. */
export const declaredModesSource = `
let count = 0;
const id = (n: number): number => {
  count = count + 1;
  return n;
};
const boom = (): string => "";
const pick = (lazyA: () => string, b: string): string => b;
const f = (a: number, lazyB: () => number, b: number, memoC: number): number[] => {
  const out: number[] = [a];
  out.push(lazyB());
  out.push(lazyB());
  out.push(b);
  out.push(force(memoC));
  out.push(force(memoC));
  return out;
};
const g = (t: () => number): number[] => [t(), t()];
const h = (t: number): number[] => [force(t), force(t)];
console.log(pick(boom, "taken"));
console.log(f(id(1), () => id(2 + 3), id(4), delay(id(5 * 6))));
console.log(count);
console.log(g(() => id(10)));
console.log(count);
console.log(h(delay(id(10))));
console.log(count);
`;

/** The pinned session through the memoized lazy experiment. */
export const answers = (): RunResult =>
  runLazySource(declaredModesSource, "lazy-memoized-experiment");

export function ex_4_31(): string {
  return (
    "The declared modes ride on the argument discipline: strict values, lazy recomputing " +
    "thunks called per demand, lazy-memo delayed cells forced per demand under the " +
    "memoized experiment. The unforced lazy parameter lets pick answer taken while its " +
    "first arm would fault on demand; f answers [1, 5, 5, 4, 30, 30] with count 5 — two " +
    "strict calls, two recomputed lazy demands, one computed lazy-memo demand; g's lazy " +
    "parameter demanded twice runs twice (count 7) and h's lazy-memo parameter demanded " +
    "twice runs once more (count 8)."
  );
}
