// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Effect, Ref } from "effect";

/**
 * Exercise 3.1a (added by this edition, extends exercise 3.1): the
 * accumulator that answers its transaction history. The running sum of
 * exercise 3.1 is kept, and every call also answers the history of
 * transactions that produced it: the amounts added so far, in order,
 * alongside the sum. The state is one Ref holding both, so the sum and
 * the history can never disagree about how many calls happened.
 */

/** What one accumulator call answers: the sum so far and the ordered
 * amounts that produced it. */
export type AccumulatorHistory = {
  readonly sum: number;
  readonly transactions: ReadonlyArray<number>;
};

/** An accumulator answering the running history. */
export type HistoryAccumulator = (amount: number) => Effect.Effect<AccumulatorHistory>;

/** Builds an accumulator whose sum starts at `initial` and whose
 * history starts empty. */
export const makeAccumulatorWithHistory = (initial: number): HistoryAccumulator => {
  const state = Ref.makeUnsafe<AccumulatorHistory>({
    sum: initial,
    transactions: [],
  });
  return (amount) =>
    Ref.modify(state, (current) => {
      const next: AccumulatorHistory = {
        sum: current.sum + amount,
        transactions: [...current.transactions, amount],
      };
      return [next, next];
    });
};
