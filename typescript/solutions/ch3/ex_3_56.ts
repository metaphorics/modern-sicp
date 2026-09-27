// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  merge,
  type StreamCell,
  scaleStream,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.56: the statement asks for the stream S of the positive
 * integers with no prime factors other than 2, 3, or 5, enumerated in
 * ascending order with no repetitions, built from the facts that S
 * begins with 1, that the elements of (scale-stream S 2), (scale-stream
 * S 3), and (scale-stream S 5) are also elements of S, and that these
 * are all the elements of S. The statement's merge combines two
 * ordered streams into one ordered result, eliminating repetitions,
 * and the missing expressions in
 *
 *   (define S (cons-stream 1 (merge (blank) (blank))))
 *
 * are the scale-of-S branches, nested pairwise:
 *
 *   (define S
 *     (cons-stream 1
 *       (merge (scale-stream S 2)
 *              (merge (scale-stream S 3)
 *                     (scale-stream S 5)))))
 */

/** The statement's S: the Hamming numbers, 1 together with the merge
 * of its own doublings, triplings, and quintuplings. */
export const S: StreamCell<number> = consStream(1, () =>
  merge(scaleStream(S, 2), merge(scaleStream(S, 3), scaleStream(S, 5))),
);
