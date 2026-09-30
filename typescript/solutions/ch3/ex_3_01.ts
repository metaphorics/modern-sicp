// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Effect, Ref } from "effect";

/**
 * Exercise 3.1: make-accumulator. The accumulator's local state is the
 * edition's `Ref` (the section module carries the argument for that
 * rendering of assignment); each generated accumulator owns one
 * Ref, so two accumulators never share a sum. A call adds its argument
 * and answers the accumulated sum so far, the book's `begin` of
 * increment-and-return rendered as one atomic `Ref.modify`.
 */

/** An accumulator: called with an amount, answers the sum so far. */
export type Accumulator = (amount: number) => Effect.Effect<number>;

/** Builds an accumulator whose sum starts at `initial`. */
export const makeAccumulator = (initial: number): Accumulator => {
  const sum = Ref.makeUnsafe(initial);
  return (amount) => Ref.modify(sum, (s) => [s + amount, s + amount]);
};
