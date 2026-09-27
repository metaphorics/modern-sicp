// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  type Stream,
  signChangeDetector,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.74: Alyssa's zero crossings through the generalized
 * stream-map of exercise 3.50. Her recursive definition pairs each
 * sample with the value before it and runs the two-argument
 * `sign-change-detector`:
 *
 *   (define (make-zero-crossings input-stream last-value)
 *     (cons-stream
 *      (sign-change-detector (stream-car input-stream) last-value)
 *      (make-zero-crossings (stream-cdr input-stream)
 *                           (stream-car input-stream))))
 *
 *   (define zero-crossings (make-zero-crossings sense-data 0))
 *
 * Eva Lu Ator observes this is approximately a map of
 * `sign-change-detector` over two streams, and the exercise asks for
 * the missing second stream. It is the sense data shifted right by one
 * sample with the initial last-value 0 padded on the front, which is
 * exactly the previous-value sequence her recursion carries.
 */

/** The book's `sense-data`: the sensor signal the text displays as
 * ... 1 2 1.5 1 0.5 -0.1 -2 -3 -2 -0.5 0.2 3 4 ... . Built head first
 * by folding the sample list from the right over `consStream`. */
export const senseData: Stream<number> = [
  1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4,
].reduceRight<Stream<number>>((rest, head) => consStream(head, () => rest), null);

/** The book's completed `zero-crossings`: `sign-change-detector` over
 * the input and the same input delayed by one sample, 0 padded in
 * front. Each output element is the crossing state of one sample
 * against its predecessor, so the output has one element per input
 * sample and ends where the input ends. */
export const zeroCrossings = (s: Stream<number>): Stream<number> =>
  streamMap2(
    signChangeDetector,
    s,
    consStream(0, () => s),
  );
