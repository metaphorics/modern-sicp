// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  type Delayed,
  force,
  type Stream,
  streamCar,
  streamCdr,
  streamMap,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.68: Louis Reasoner thinks that building a stream of pairs
 * from three parts is unnecessarily complicated. Instead of separating
 * the pair (S0, T0) from the rest of the pairs in the first row, he
 * proposes to work with the whole first row, appending it to the
 * recursively defined rest:
 *
 *   pairs(s, t) = streamAppend(streamMap(x -> (streamCar s, x), t),
 *                               pairs(streamCdr s, streamCdr t))
 *
 * Does this work? The demonstration answers by execution with a step
 * budget: a meter in the row map throws past 200 steps, and the two
 * shapes of the failure both land on it. With the rest delayed, the
 * append must finish the infinite first row before the rest can
 * contribute one element, so the stream never yields anything from its
 * second component. With the rest spelled as the statement's plain
 * argument, applicative order evaluates the recursion before the
 * append can serve anything at all.
 */

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
export const makeLouisMeter = (budget: number): LouisMeter => {
  let count = 0;
  return {
    tick: () => {
      count += 1;
      if (count > budget) {
        throw new LouisBudgetExceeded(count, budget);
      }
    },
    steps: () => count,
  };
};

/** The book's `streamAppend`: the elements of s1 followed by the
 * elements of s2, empties passing through. The recursive call sits
 * behind consStream's delay, so s2 is reached only when s1 runs
 * empty. */
export const streamAppend = <A>(s1: Stream<A>, s2: Stream<A>): Stream<A> =>
  s1 === null ? s2 : consStream(s1.head, () => streamAppend(streamCdr(s1), s2));

/** The same append with the rest spelled as the delayed argument,
 * matching consStream's discipline: s2 is forced only when s1 runs
 * empty, so appending an infinite first row to a recursively defined
 * rest is expressible without evaluating the rest eagerly. This is the
 * spelling that gives Louis's definition its best case. */
export const streamAppendDelayed = <A>(s1: Stream<A>, s2: Delayed<Stream<A>>): Stream<A> =>
  s1 === null ? force(s2) : consStream(s1.head, () => streamAppendDelayed(streamCdr(s1), s2));

/** Louis's mapped whole first row: (streamCar s, x) for every x in t,
 * with the meter in the per-element closure. */
const meteredFirstRow = (meter: LouisMeter, s: Stream<number>, t: Stream<number>): Stream<Pair> =>
  streamMap((x): Pair => {
    meter.tick();
    return [streamCar(s), x];
  }, t);

/** Louis's pairs, whole first row and all, the rest passed as the
 * delayed second argument. The append serves the first row forever and
 * never reaches the rest. */
export const louisPairs = (
  meter: LouisMeter,
  s: Stream<number>,
  t: Stream<number>,
): Stream<Pair> => {
  const go = (a: Stream<number>, b: Stream<number>): Stream<Pair> =>
    streamAppendDelayed(meteredFirstRow(meter, a, b), () => go(streamCdr(a), streamCdr(b)));
  return go(s, t);
};

/** Louis's pairs in the statement's plain argument shape: the
 * recursive rest is an ordinary argument, evaluated before the append
 * can serve anything, so the construction itself dives through the
 * rows until the meter aborts it. */
export const louisPairsPlainArgument = (
  meter: LouisMeter,
  s: Stream<number>,
  t: Stream<number>,
): Stream<Pair> => {
  const go = (a: Stream<number>, b: Stream<number>): Stream<Pair> =>
    streamAppend(meteredFirstRow(meter, a, b), go(streamCdr(a), streamCdr(b)));
  return go(s, t);
};
