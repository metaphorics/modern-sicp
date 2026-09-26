// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type Stream,
  signChangeDetector,
  streamCdr,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.76: Eva Lu Ator's modularity criticism of Louis's 3.75
 * program: it intermixes smoothing with zero-crossing extraction, so
 * the extractor would have to change if Alyssa conditioned her signal
 * differently. The fix is to write `smooth` as a component and build
 * the detector from it:
 *
 *   (define (smooth s)
 *     (stream-map (lambda (x y) (/ (+ x y) 2)) s (stream-cdr s)))
 *
 *   (define (zero-crossings s)
 *     (stream-map sign-change-detector
 *                 (smooth s)
 *                 (smooth (stream-cdr s))))
 *
 * with the map pairing each smoothed point with its successor.
 */

/** The book's `smooth`: each element is the average of two successive
 * elements of `s`, so a finite input answers one element fewer and an
 * infinite input answers a stream of the same length. */
export const smooth = (s: Stream<number>): Stream<number> =>
  streamMap2((x, y) => (x + y) / 2, s, streamCdr(s));

/** The zero-crossing detector rewritten from components. The module's
 * `signChangeDetector` reads (current, previous), so the later
 * smoothed point, `smooth` of the tail, supplies the current sample
 * and `smooth` of the whole stream supplies the previous one; pairing
 * them the other way round would run the detector backwards in time
 * and flip the sign of every crossing. Against the padded fix of
 * exercise 3.75 this composition answers the crossings of the same
 * smoothed signal without its two boundary elements (the comparison
 * against the initial 0 and the first smoothed-to-smoothed step). */
export const zeroCrossings = (s: Stream<number>): Stream<number> =>
  streamMap2(signChangeDetector, smooth(streamCdr(s)), smooth(s));
