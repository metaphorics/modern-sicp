// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  integers,
  ones,
  type Stream,
  streamCar,
  streamCdr,
  streamIsNull,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.50: the multi-stream `streamMap`. The book's skeleton is
 * variadic: `streamMap` takes a procedure and any number of streams,
 * stops when the streams run out, and applies the procedure to the
 * cars while recurring on the cdrs. This edition spells the argument
 * list as rest parameters and the two book blanks as the questions the
 * skeleton leaves open: the emptiness test sees every stream, and the
 * combination applies the procedure across all the cars.
 */

/** The book's `streamMap` completed: element-wise `proc` over any
 * number of streams, ending where the shortest ends. */
export const streamMapN = <A, B>(
  proc: (...args: A[]) => B,
  ...argstreams: Stream<A>[]
): Stream<B> => {
  if (argstreams.length === 0 || argstreams.some((s) => streamIsNull(s))) {
    return null;
  }
  return consStream(proc(...argstreams.map(streamCar)), () =>
    streamMapN(proc, ...argstreams.map(streamCdr)),
  );
};

/** The book's `addStreams` built on the completed map. */
export const addStreamsN = (s1: Stream<number>, s2: Stream<number>): Stream<number> =>
  streamMapN((a, b) => a + b, s1, s2);

/** The text's check: the integers must be the ones plus the integers
 * again, now computed through the exercise's map. */
export const integersThroughMapN: Stream<number> = consStream(1, () => addStreamsN(ones, integers));
