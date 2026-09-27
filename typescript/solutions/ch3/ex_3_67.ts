// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  interleave,
  type Stream,
  streamCdr,
  streamMap,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.67: modify `pairs` so that (pairs integers integers)
 * produces the stream of all pairs (i, j), without the condition
 * i <= j. The hint says to mix in an additional stream: the modified
 * stream is the head pair, then the interleave of three parts, the
 * rest of the first row (S0, T1), (S0, T2), ..., the first column
 * below the head (S1, T0), (S2, T0), ..., and the recursive all-pairs
 * of the cdrs. Every pair now appears in both orders.
 */

/** The book's modified `pairs`: every pair of S and T, both orders. */
export const allPairs = <A>(s: Stream<A>, t: Stream<A>): Stream<[A, A]> =>
  s === null || t === null
    ? null
    : consStream([s.head, t.head], () =>
        interleave(
          streamMap((x) => [s.head, x] as [A, A], streamCdr(t)),
          interleave(
            streamMap((x) => [x, t.head] as [A, A], streamCdr(s)),
            allPairs(streamCdr(s), streamCdr(t)),
          ),
        ),
      );

/** The exercise-3.66 position walker, restated rather than imported
 * across exercise files: the 0-based position of the first pair equal
 * to `target`, or -1 if none appears within `limit` elements. */
export const positionOfPair = (
  s: Stream<[number, number]>,
  target: readonly [number, number],
  limit: number,
): number => {
  let rest = s;
  for (let i = 0; i < limit; i += 1) {
    if (rest === null) {
      return -1;
    }
    if (rest.head[0] === target[0] && rest.head[1] === target[1]) {
      return i;
    }
    rest = streamCdr(rest);
  }
  return -1;
};
