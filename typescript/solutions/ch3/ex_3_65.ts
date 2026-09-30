// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  accelerateSequence,
  addStreams,
  consStream,
  eulerTransform,
  type Stream,
  type StreamCell,
  streamCdr,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.65: the alternating series ln 2 = 1 - 1/2 + 1/3 - 1/4 +
 * ... gives three sequences of approximations: the partial sums
 * themselves, their euler transform, and the accelerated sequence of
 * the tableau's first column. The statement's question, "how rapidly
 * do these sequences converge?", is answered with counted
 * `streamLimit` searches: at tolerance 0.0001 the plain partial sums
 * need about ten thousand terms, the euler transform 13, the
 * accelerated sequence 5.
 */

/**
 * The statement's `ln2Summands`: 1, -1/2, 1/3, -1/4, ... The
 * shape is `consStream(1 / n, () => streamMap((x) => -x,
 * ln2Summands(n + 1)))`: each element wraps one more `streamMap`
 * negation layer around the recursive rest, so element k sits k map
 * cells deep. With memoized tails every cell is still computed once,
 * but the allocation is quadratic in the prefix length (measured: 251
 * cells per element at element 500, and a 10000-element walk of the
 * partial sums below allocating 12.4 GB in 17 s, enough to kill a
 * worker). This generator carries the sign as a parameter instead and
 * produces the identical sequence with one cell per element.
 */
export const ln2Summands = (n: number): StreamCell<number> => ln2SummandsSigned(n, 1);

const ln2SummandsSigned = (n: number, sign: number): StreamCell<number> =>
  consStream(sign / n, () => ln2SummandsSigned(n + 1, -sign));

/**
 * The book's `ln2Stream`: the partial sums of the summands. The
 * stream is bound locally and referenced by its own tail, the
 * exercise-3.63 shape: each partial sum is then computed once from the
 * one before. The module's `partialSums` re-invokes itself on every
 * tail force, which makes the ten-thousand-term walk this exercise's
 * comparison needs cost quadratic time (measured: 14.6 s to walk the
 * first 10000 harmonic partial sums, against 2 ms for this locally
 * bound shape, element values identical), so the exercise's own stream
 * takes the local binding.
 */
export const ln2Stream = (): StreamCell<number> => {
  const summands = ln2Summands(1);
  const sums: StreamCell<number> = consStream(summands.head, () =>
    addStreams(sums, streamCdr(summands)),
  );
  return sums;
};

/** The first acceleration step over the partial sums. */
export const ln2EulerStream = (sums: Stream<number>): Stream<number> => eulerTransform(sums);

/** The tableau's first column: the fully accelerated sequence. */
export const ln2AcceleratedStream = (sums: Stream<number>): Stream<number> =>
  accelerateSequence(eulerTransform, sums);

/** The exercise-3.64 stream limit with its term count made visible;
 * restated here rather than imported across exercise files. */
export interface LimitWithTerms {
  readonly value: number;
  readonly terms: number;
}

/** Walks until two successive elements differ by less than the
 * tolerance, returning the second and the number of elements examined. */
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
