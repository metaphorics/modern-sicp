// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { integers, pairs, type Stream, streamCdr } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.66: examining the stream (pairs integers integers). The
 * statement asks for general comments about the order in which the
 * pairs appear, with the examples (1, 100), (99, 100), (100, 100).
 * The answer below is read off executed walks. The structure: the
 * stream starts with (1, 1); its tail interleaves the whole first row
 * (1, 2), (1, 3), ... against the recursive pairs of the cdrs, so
 * every deeper row is interleaved against everything before it. The
 * measured consequences: the diagonal pair (i, i) sits at position
 * 2^i - 2; row 1 is fully spread out, with (1, j) at position 2j - 3,
 * so (1, 100) is at 197, the book's "about 198 pairs"; row i's spacing
 * doubles to 2^i per column step once the row clears the diagonal
 * clutter, so near-diagonal pairs like (99, 100) appear only after
 * about 2^99 predecessors, and (100, 100) after about 2^100.
 */

/** The stream the statement examines. */
export const intPairs: Stream<[number, number]> = pairs(integers, integers);

/** The 0-based position of the first pair equal to `target`, or -1 if
 * none appears within the first `limit` elements. The limit keeps the
 * search finite on a stream whose deep positions grow as 2^i. */
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
