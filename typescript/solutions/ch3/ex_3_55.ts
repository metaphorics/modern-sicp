// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  addStreams,
  consStream,
  type Stream,
  streamCdr,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.55: the statement asks for a procedure `partial-sums`
 * that takes a stream S and answers the stream of S0, S0 + S1,
 * S0 + S1 + S2, ...; for example, `(partial-sums integers)` should be
 * the stream 1, 3, 6, 10, 15, ... The book's own shape is a
 * consStream of the head and the sum of the partial sums with the
 * rest of S, defined in terms of itself:
 *
 *   const partialSums = (s) =>
 *     consStream(s.head, () => addStreams(partialSums(s), streamCdr(s)));
 */

/** The statement's `partialSums`: the running total of the elements,
 * each element the previous total plus the next element of `s`. */
export const partialSumsEx = (s: Stream<number>): Stream<number> =>
  s === null ? null : consStream(s.head, () => addStreams(partialSumsEx(s), streamCdr(s)));
