// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  isMNil,
  type MCons,
  type MList,
  mlist,
  setCdr,
} from "../../packages/ch3/src/03-mutable-data.js";
import { lastPair } from "./ex_3_12.js";

/**
 * Exercise 3.13: make-cycle. The book closes the ring by rewriting
 * the final pair's tail to point at the first pair, so the structure
 * has no empty end: walking tails loops forever, and so would
 * printing it.
 */

/** The book's `make-cycle`: mutates the final pair of `x` so its
 * tail is `x` itself, and returns `x`. The result is a ring:
 * `showMList` on it never terminates, so the tests pin it by identity
 * only. */
export const makeCycle = <A>(x: MList<A>): MCons<A> => {
  if (isMNil(x)) {
    throw new Error("make-cycle: empty list");
  }
  setCdr(lastPair(x), x);
  return x;
};

/** The identity pins of the book's ring over (a b c): the heads met
 * on the way around, and whether the third pair's tail is the first
 * pair again. */
export interface CycleDemo {
  readonly firstHead: string;
  readonly secondHead: string;
  readonly thirdHead: string;
  readonly thirdTailsBackToFirst: boolean;
}

const tailPair = <A>(l: MList<A>): MCons<A> => {
  if (isMNil(l)) {
    throw new Error("cycle demo: expected a pair");
  }
  return l;
};

/** Walks the book's ring two tails from the first pair and reports
 * the heads plus the identity pin; never prints the ring. */
export const cycleDemo = (): CycleDemo => {
  const first = makeCycle(mlist("a", "b", "c"));
  const second = tailPair(first.tail);
  const third = tailPair(second.tail);
  return {
    firstHead: first.head,
    secondHead: second.head,
    thirdHead: third.head,
    thirdTailsBackToFirst: Object.is(third.tail, first),
  };
};
