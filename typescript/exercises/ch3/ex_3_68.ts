// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Delayed, Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.68: Louis Reasoner thinks that building a stream of pairs
 * from three parts is unnecessarily complicated. Instead of separating
 * the pair (S0, T0) from the rest of the pairs in the first row, he
 * proposes to work with the whole first row, appending it to the
 * recursively defined rest. Does this work? Consider what happens if
 * we evaluate louisPairs over the integers. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_68.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.68 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One pair of integers, the elements Louis's stream is made of. */
export type Pair = [number, number];

/** The named abort raised when the metered row map passes its budget. */
export class LouisBudgetExceeded extends Error {
  constructor(steps: number, budget: number) {
    super(`louis pairs passed its ${budget}-step budget at step ${steps}`);
    this.name = "LouisBudgetExceeded";
  }
}

/** The step meter of the demonstration: one tick per first-row element
 * computed, aborting once the budget is gone. */
export interface LouisMeter {
  tick(): void;
  steps(): number;
}

/** A meter with the given step budget. */
export function makeLouisMeter(_budget: number): LouisMeter {
  throw new PendingSolution();
}

/** The book's `stream-append`: the elements of s1 followed by the
 * elements of s2, empties passing through. */
export function streamAppend<A>(_s1: Stream<A>, _s2: Stream<A>): Stream<A> {
  throw new PendingSolution();
}

/** The same append with the rest spelled as the delayed argument, so
 * appending an infinite first row to a recursively defined rest is
 * expressible without evaluating the rest eagerly. */
export function streamAppendDelayed<A>(_s1: Stream<A>, _s2: Delayed<Stream<A>>): Stream<A> {
  throw new PendingSolution();
}

/** Louis's pairs, whole first row and all, the rest passed as the
 * delayed second argument. */
export function louisPairs(
  _meter: LouisMeter,
  _s: Stream<number>,
  _t: Stream<number>,
): Stream<Pair> {
  throw new PendingSolution();
}

/** Louis's pairs in the statement's plain argument shape, where the
 * recursive rest is evaluated before the append can serve anything. */
export function louisPairsPlainArgument(
  _meter: LouisMeter,
  _s: Stream<number>,
  _t: Stream<number>,
): Stream<Pair> {
  throw new PendingSolution();
}
