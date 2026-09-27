// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  integers,
  type Stream,
  type StreamCell,
  streamCdr,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.54: the statement asks for a procedure `mul-streams`,
 * analogous to `add-streams`, that produces the elementwise product
 * of its two input streams, and then to complete
 *
 *   (define factorials
 *     (cons-stream 1 (mul-streams (blank) (blank))))
 *
 * so that the n-th element (counting from 0) is (n + 1) factorial.
 * The blanks are the integers with their first element dropped and
 * the factorials themselves: element n of the product is (n + 2)
 * times factorials[n], and factorials[0] is the stated 1, so the
 * definition closes over itself exactly like the text's `integers`.
 */

/** The book's `mul-streams`: element-wise product, ending where
 * either input ends. */
export const mulStreams = (s1: Stream<number>, s2: Stream<number>): Stream<number> =>
  streamMap2((a, b) => a * b, s1, s2);

/** The statement's completed definition: 1, then the elementwise
 * product of 2, 3, 4, ... with the factorials themselves, so element
 * n counting from 0 is (n + 1)!. */
export const factorials: StreamCell<number> = consStream(1, () =>
  mulStreams(factorials, streamCdr(integers)),
);
