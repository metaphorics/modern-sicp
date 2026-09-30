// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  accelerateSequence,
  eulerTransform,
  type Stream,
  sqrtStream,
  streamCar,
  streamCdr,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.64: `streamLimit` examines a stream until two successive
 * elements differ in absolute value by less than the tolerance, and
 * returns the second of the two. With it, square roots come from the
 * module's `sqrtStream`: `(sqrt x tolerance)` is `streamLimit` applied
 * to the Newton guesses. The counted variant reports how many elements
 * the search examined, which this exercise and the map's tailored idea
 * (comparing sequence accelerators) need for their numbers.
 */

/** The book's `streamLimit`: the first successor closer than
 * `tolerance` to its predecessor. */
export const streamLimit = (s: Stream<number>, tolerance: number): number => {
  if (s === null) {
    throw new Error("streamLimit: the stream ran out before it converged");
  }
  const rest = streamCdr(s);
  if (rest === null) {
    throw new Error("streamLimit: the stream ran out before it converged");
  }
  if (Math.abs(streamCar(rest) - streamCar(s)) < tolerance) {
    return streamCar(rest);
  }
  return streamLimit(rest, tolerance);
};

/** The exercise's `sqrt` built on the module's Newton stream. */
export const sqrtWithin = (x: number, tolerance: number): number =>
  streamLimit(sqrtStream(x), tolerance);

/** The limit answer together with the number of elements examined,
 * which is the convergence-rate measurement this section compares. */
export interface LimitWithTerms {
  readonly value: number;
  readonly terms: number;
}

/** The book's `streamLimit` walking iteratively so long searches stay
 * flat on the stack, counting the elements it passed. */
export const streamLimitWithTerms = (s: Stream<number>, tolerance: number): LimitWithTerms => {
  let rest = s;
  let terms = 1;
  for (;;) {
    if (rest === null) {
      break;
    }
    const next = streamCdr(rest);
    if (next === null) {
      break;
    }
    if (Math.abs(next.head - rest.head) < tolerance) {
      return { value: next.head, terms: terms + 1 };
    }
    rest = next;
    terms += 1;
  }
  throw new Error("streamLimit: the stream ran out before it converged");
};

/** The map's tailored comparison: Newton's stream pushed through the
 * section's euler acceleration and tableau, the way the ln 2 series is
 * accelerated in exercise 3.65. */
export const acceleratedSqrtStream = (x: number): Stream<number> =>
  accelerateSequence(eulerTransform, sqrtStream(x));

/** The accelerated stream's limit double, one ulp below Math.SQRT2:
 * the same measured value exercise 3.63 pins, restated here because
 * exercises do not import across solution files. */
// biome-ignore lint/suspicious/noApproximativeNumericConstant: the measured value differs from Math.SQRT2 by one ulp
export const sqrt2AcceleratedLimit = 1.414213562373095;
