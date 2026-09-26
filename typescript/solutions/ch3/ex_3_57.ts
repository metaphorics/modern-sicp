// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  type StreamCell,
  streamCdr,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.57: the statement asks how many additions are performed
 * computing the n-th Fibonacci number with the text's fibs based on
 * add-streams,
 *
 *   (define fibs
 *     (cons-stream
 *      0 (cons-stream
 *         1 (add-streams
 *            (stream-cdr fibs) fibs))))
 *
 * and to show the count would be exponentially greater had delay been
 * just (lambda () exp) instead of the section's memo-proc. The answer
 * the instrumented streams below measure: under memoized delay,
 * elements 0 and 1 are given and each later element costs exactly one
 * addition, so reaching element n costs n - 1 additions. Under plain
 * thunks, every re-reference to a tail re-runs it, each element's
 * substream is rebuilt once per later demand, and the addition count
 * itself obeys the Fibonacci recurrence, growing exponentially in n.
 */

/** A tally the instrumented streams report additions into. */
export interface AdditionCounter {
  additions: number;
}

/** The text's add-streams fibs, instrumented: each element-wise
 * addition bumps the counter. The memoized tails of `consStream` make
 * each element's addition happen once, so streamRef to index n
 * reports n - 1. */
export const fibsCounting = (counter: AdditionCounter): StreamCell<number> => {
  const add = (a: number, b: number): number => {
    counter.additions += 1;
    return a + b;
  };
  const stream: StreamCell<number> = consStream(0, () =>
    consStream(1, () => streamMap2(add, streamCdr(stream), stream)),
  );
  return stream;
};

/** The book's cons-stream with delay spelled as a plain thunk: the
 * same pair shape without the memo-proc wrapping, the alternative the
 * statement asks about. */
export interface PlainStreamCell<A> {
  readonly head: A;
  readonly tail: () => PlainStream<A>;
}

/** A plain-thunk stream: cells ending in the empty stream, `null`,
 * whose tails recompute on every reference. */
export type PlainStream<A> = PlainStreamCell<A> | null;

/** The book's (cons-stream a b) with (delay b) as (lambda () b). */
export const consPlain = <A>(head: A, tail: () => PlainStream<A>): PlainStreamCell<A> => ({
  head,
  tail,
});

/** stream-cdr for plain cells: re-runs the tail every time. */
const cdrPlain = <A>(s: PlainStream<A>): PlainStream<A> => (s === null ? null : s.tail());

/** The same instrumented fibs built on plain thunks: identical shape,
 * identical single addition per map step, no memoization, so
 * streamRef to index n re-forces tails down a Fibonacci-shaped tree of
 * recomputations and the counter grows exponentially. */
export const fibsPlainCounting = (counter: AdditionCounter): PlainStreamCell<number> => {
  const add = (a: number, b: number): number => {
    counter.additions += 1;
    return a + b;
  };
  const addStreamsPlain = (
    s1: PlainStream<number>,
    s2: PlainStream<number>,
  ): PlainStream<number> =>
    s1 === null || s2 === null
      ? null
      : consPlain(add(s1.head, s2.head), () => addStreamsPlain(cdrPlain(s1), cdrPlain(s2)));
  const stream: PlainStreamCell<number> = consPlain(0, () =>
    consPlain(1, () => addStreamsPlain(cdrPlain(stream), stream)),
  );
  return stream;
};
