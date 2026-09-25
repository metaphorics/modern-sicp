// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

/**
 * Exercise 3.6: rand answering generate and reset requests, so
 * sequences can be replayed. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_06.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.6 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds a generator whose sequence resets to any value on demand. */
export function makeResettableRand(_seed: number): (request: RandRequest) => Effect.Effect<number> {
  throw new PendingSolution();
}

/** The request union: the book's 'generate and 'reset messages. */
export type RandRequest =
  | { readonly _tag: "Generate" }
  | { readonly _tag: "Reset"; readonly newValue: number };
