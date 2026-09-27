// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Effect, Ref } from "effect";

import { randUpdate } from "../../packages/ch3/src/01-assignment.js";

/**
 * Exercise 3.6: a rand that answers 'generate or 'reset. The book's
 * two symbols become the members of the request union; the hidden
 * state is one Ref, so a reset re-seeds the very cell generate draws
 * from and the repeatable-sequence property is structural, not
 * promised. A reset answers the state it installed (the book leaves
 * the reset call's value unspecified).
 */

/** A request to the resettable generator: the book's 'generate and
 * '((reset) new-value) messages. */
export type RandRequest =
  | { readonly _tag: "Generate" }
  | { readonly _tag: "Reset"; readonly newValue: number };

/** The resettable generator: each request answers a number, the fresh
 * state for a reset. */
export type ResettableRand = (request: RandRequest) => Effect.Effect<number>;

/** Builds the generator over the section's `rand-update`, seeded. */
export const makeResettableRand = (seed: number): ResettableRand => {
  const x = Ref.makeUnsafe(seed);
  return (request) =>
    request._tag === "Generate"
      ? Ref.modify(x, (current) => {
          const next = randUpdate(current);
          return [next, next];
        })
      : Ref.setAndGet(x, request.newValue);
};
