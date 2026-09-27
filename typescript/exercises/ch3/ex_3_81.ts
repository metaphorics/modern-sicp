// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.81: the request-driven `rand` stream. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_81.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.81 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One request to the generator: this edition's shape of the book's
 * `(generate)` and `(reset <new-value>)` messages. */
export type RandRequest = "generate" | { readonly kind: "reset"; readonly seed: number };

/** The book's request-driven `rand`: the stream of random numbers
 * answering the request stream, starting from `random-init`, with no
 * assignment anywhere. */
export function randStream(_requests: Stream<RandRequest>): Stream<number> {
  throw new PendingSolution();
}
