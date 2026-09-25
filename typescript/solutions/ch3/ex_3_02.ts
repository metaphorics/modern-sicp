// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Ref } from "effect";

/**
 * Exercise 3.2: make-monitored. The book's special symbols
 * 'how-many-calls? and 'reset-count become members of a closed request
 * union, the edition's standing replacement for message symbols (2.73):
 * the compiler holds every caller to the three requests, so the book's
 * "any other input" arm is the typed Call member. The counter is one
 * Ref captured by the monitored procedure, and the calls counted are
 * the calls answered: an argument is passed to f, its result returned,
 * and the counter incremented after the result is in hand.
 */

/** A request to a monitored procedure: call f, or ask about the
 * counter. */
export type MonitorRequest<A> =
  | { readonly _tag: "Call"; readonly arg: A }
  | { readonly _tag: "HowManyCalls" }
  | { readonly _tag: "ResetCount" };

/** A monitored procedure: Call answers f's result, the two bookkeeping
 * requests answer counts (a reset answers 0, the counter's value after
 * the reset). */
export type Monitored<A, B> = (request: MonitorRequest<A>) => Effect.Effect<B | number>;

/** Wraps `f` (a one-argument procedure) in a monitored procedure. */
export const makeMonitored = <A, B>(f: (arg: A) => B): Monitored<A, B> => {
  const count = Ref.makeUnsafe(0);
  return (request) =>
    Effect.gen(function* () {
      if (request._tag === "Call") {
        const value = f(request.arg);
        yield* Ref.update(count, (n) => n + 1);
        return value;
      }
      if (request._tag === "HowManyCalls") {
        return yield* Ref.get(count);
      }
      return yield* Ref.setAndGet(count, 0);
    });
};
