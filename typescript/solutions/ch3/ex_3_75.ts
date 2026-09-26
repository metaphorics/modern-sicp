// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  type Stream,
  signChangeDetector,
  streamCdr,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.75: Louis Reasoner's smoothing bug. Alyssa's plan is to
 * detect crossings on the signal smoothed by averaging each sample
 * with the previous one. Louis altered her program to
 *
 *   (define (make-zero-crossings input-stream last-value)
 *     (let ((avpt (/ (+ (stream-car input-stream) last-value) 2)))
 *       (cons-stream
 *        (sign-change-detector avpt last-value)
 *        (make-zero-crossings (stream-cdr input-stream) avpt))))
 *
 * The plan needs two carried facts: the previous raw sample, to form
 * the next average with, and the previous smoothed average, to compare
 * the new average against. Louis's version keeps only the one slot
 * `last-value`, and it holds the raw sample where the detector call
 * needs the previous smoothed value: he passes the carried raw value
 * as the detector's `previous` and then overwrites the slot with
 * `avpt`, so the raw history is lost. The smoothing decays into an
 * exponential average that hands each new raw sample's sign down at
 * half strength, and the detector, fed that half-raw residue instead
 * of the previous smoothed point, fires on noise and can miss
 * crossings of the signal Alyssa actually specified.
 */

/** Louis's version, translated exactly as the statement prints it:
 * `avpt` averages the raw sample with the carried slot, the detector
 * compares `avpt` against that same slot, and the recursion carries
 * `avpt` on in place of the raw previous value. */
export const louisZeroCrossings = (
  inputStream: Stream<number>,
  lastValue: number,
): Stream<number> => {
  if (inputStream === null) {
    return null;
  }
  const avpt = (inputStream.head + lastValue) / 2;
  return consStream(signChangeDetector(avpt, lastValue), () =>
    louisZeroCrossings(streamCdr(inputStream), avpt),
  );
};

/** The fix, Louis's structure with the hint's extra argument: `avpt`
 * still averages the current raw sample with the previous raw sample,
 * the detector compares it with the previous smoothed value
 * `lastAvpt`, and the recursion carries the raw and the smoothed
 * values separately. */
export const makeZeroCrossingsSmoothed = (
  inputStream: Stream<number>,
  lastValue: number,
  lastAvpt: number,
): Stream<number> => {
  if (inputStream === null) {
    return null;
  }
  const avpt = (inputStream.head + lastValue) / 2;
  return consStream(signChangeDetector(avpt, lastAvpt), () =>
    makeZeroCrossingsSmoothed(streamCdr(inputStream), inputStream.head, avpt),
  );
};

/** Alyssa's plan assembled: the zero crossings of the pairwise-smoothed
 * signal, starting from 0 as both the previous value and the previous
 * average, the same seed Alyssa's original used. */
export const zeroCrossingsSmoothed = (s: Stream<number>): Stream<number> =>
  makeZeroCrossingsSmoothed(s, 0, 0);
