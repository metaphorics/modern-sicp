// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { randomInit, randUpdate } from "../../packages/ch3/src/01-assignment.js";
import { consStream, type Stream, streamCdr } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.81: the request-driven `rand` stream. Exercise 3.6
 * generalized the random-number generator so it could be reset to a
 * specified value, giving repeatable sequences of "random" numbers;
 * this exercise asks for the same service as a stream process: an
 * input stream of requests, each either to `generate` a new number or
 * to `reset` the sequence to a specified value, and an output stream
 * of the random numbers, with no assignment anywhere. The seed is
 * carried as a value instead of in a state cell: `generate` emits
 * `rand-update` of the carried seed and `reset` emits the requested
 * seed itself, and the tail of the output recurses on the rest of the
 * requests with the emitted value as the new carried seed. The
 * generator's state therefore lives in the stream of emitted numbers,
 * which is exactly why a reset makes the following subsequence a
 * replay of a fresh start.
 */

/** One request to the generator: this edition's shape of the book's
 * `(generate)` and `(reset <new-value>)` messages. */
export type RandRequest = "generate" | { readonly kind: "reset"; readonly seed: number };

/** The book's request-driven `rand`: the stream of random numbers
 * answering the request stream, starting from `random-init`. Passing
 * `seed` overrides the starting state, which the recursion uses to
 * carry the current state down the stream. Each request answers with
 * the next xorshift state for `generate` or the requested seed for
 * `reset`, and the answer doubles as the seed carried to the tail. */
export const randStream = (
  requests: Stream<RandRequest>,
  seed: number = randomInit,
): Stream<number> => {
  if (requests === null) {
    return null;
  }
  const req = requests.head;
  const value = typeof req === "string" ? randUpdate(seed) : req.seed;
  return consStream(value, () => randStream(streamCdr(requests), value));
};
