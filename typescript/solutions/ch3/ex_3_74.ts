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
 * streamMap of exercise 3.50. Her recursive definition pairs each
 * sample with the value before it and runs the two-argument
 * `signChangeDetector`:
 *
 *   const makeZeroCrossings = (inputStream, lastValue) =>
 *     consStream(signChangeDetector(inputStream.head, lastValue),
 *       () => makeZeroCrossings(streamCdr(inputStream), inputStream.head));
 *
 *   const zeroCrossings = makeZeroCrossings(senseData, 0);
 *
 * Eva Lu Ator observes this is approximately a map of
 * `signChangeDetector` over two streams, and the exercise asks for
 * the missing second stream. It is the sense data shifted right by one
 * sample with the initial last-value 0 padded on the front, which is
 * exactly the previous-value sequence her recursion carries.
 */

/** The book's `senseData`: the sensor signal the text displays as
 * ... 1 2 1.5 1 0.5 -0.1 -2 -3 -2 -0.5 0.2 3 4 ... . Built head first
 * by folding the sample list from the right over `consStream`. */
export const senseData: Stream<number> = [
  1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4,
].reduceRight<Stream<number>>((rest, head) => consStream(head, () => rest), null);

/** The book's completed `zeroCrossings`: `signChangeDetector` over
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
