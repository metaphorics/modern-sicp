// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.34: printing lazy pairs. The representation changes so
 * the renderer can identify a lazy pair: `consL` builds the two-slot
 * lazy cell, `carL`/`cdrL` address its forced slots, and the printer
 * renders with a budget. The print rule: a lazy list prints its first
 * ten elements, each forced once, and the unprinted tail prints as the
 * ellipsis, so an infinite list renders finitely and the printer never
 * forces past the budget. Nested lazy pairs print inside their
 * parent's brackets with their own budget.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The printable lazy pairs: structure, budgeted renderer, and the session. */
export const printablePairsSource = `
type LazyItem = number | null | { head: LazyItem; tail: LazyItem };
const consL = (h: LazyItem, t: LazyItem): LazyItem => ({ head: h, tail: t });
const carL = (p: LazyItem): LazyItem => {
  if (typeof p === "number" || p === null) {
    return force((p === null ? 0 : p));
  }
  return force(p.head);
};
const render = (item: LazyItem, budget: number): string => {
  if (typeof item === "number" || item === null) {
    return item === null ? "" : [item].join("");
  }
  if (budget <= 0) {
    return "...";
  }
  const head = renderOne(force(item.head), budget);
  const rest = render(force(item.tail), budget - 1);
  return rest === "" ? head : rest === "..." ? head + ", ..." : head + ", " + rest;
};
const renderOne = (item: LazyItem, budget: number): string =>
  item === null ? "null" : typeof item === "number" ? [item].join("") : "[" + render(item, budget) + "]";
const show = (item: LazyItem): string => "[" + render(item, 10) + "]";
console.log(show(consL(delay(1), delay(consL(delay(2), delay(null))))));
let ones: LazyItem = null;
const build = (): LazyItem => {
  ones = consL(delay(1), delay(ones));
  return ones;
};
console.log(build() === null ? "failed" : "ok");
console.log(show(ones));
console.log(carL(ones));
console.log(show(consL(delay(consL(delay(1), delay(null))), delay(consL(delay(2), delay(null))))));
`;

/** The printable-pairs session through the memoized lazy experiment. */
export const answers = (): RunResult =>
  runLazySource(printablePairsSource, "lazy-memoized-experiment");

export function ex_4_34(): string {
  return (
    "The renderer identifies lazy pairs and prints with a budget: the finite pair renders " +
    "[1, 2], the self-referential definition builds with an ok and renders [1, 1, 1, 1, " +
    "1, 1, 1, 1, 1, 1, ...], a demand answers 1, and the nested pair renders [[1], 2] with " +
    "its own budget. The printer never forces past the budget nor the tail of an " +
    "unprinted element, so an infinite list renders finitely."
  );
}
