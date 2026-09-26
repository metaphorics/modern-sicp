// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  type Stream,
  type StreamCell,
  sqrtImprove,
  streamMap,
  streamRef,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.63: Louis Reasoner asks why `sqrt-stream` was not written
 * without the local `guesses` variable, mapping `sqrt-improve` over a
 * fresh recursive call instead. Alyssa answers that his version
 * "performs redundant computation". The mechanism: the module's
 * `cons-stream` memoizes every tail, so Alyssa's local binding builds
 * one shared stream in which each guess is computed exactly once from
 * its predecessor. Louis's version evaluates `(sqrt-stream x)` again
 * every time a tail is forced, so each level of the map sits over a
 * fresh copy whose own tails build further copies: reaching the n-th
 * guess recomputes the k-th guess k times. The second question asks
 * whether the versions still differ if `delay` is a plain
 * `(lambda () exp)` without `memo-proc`. Measured answer below: the
 * local shape loses its advantage entirely (every re-reference
 * recomputes the whole prefix), and at the counted sizes the two
 * versions then perform the same number of improvement steps.
 */

/** A sqrt-stream builder parameterized by the Newton step, so the
 * calls to that step can be counted by the probes below. */
export type SqrtStreamMaker = (
  x: number,
  improve: (guess: number, x: number) => number,
) => StreamCell<number>;

/** Mutable counter threaded through the instrumented Newton step. */
export interface ImproveCounter {
  calls: number;
}

/** The guess the double-precision Newton iteration for sqrt(2) has
 * settled on from element 5 onward: one ulp below Math.SQRT2, which is
 * why the measured pin is spelled as a literal and not as the
 * constant. */
// biome-ignore lint/suspicious/noApproximativeNumericConstant: the measured fixed point differs from Math.SQRT2 by one ulp
export const sqrt2ConvergedGuess = 1.414213562373095;

/** Alyssa's shape (the module's own `sqrtStream`): the local `guesses`
 * binding makes the stream self-referential, so `cons-stream`'s
 * memoized tails compute each guess once. */
export const sqrtStreamLocal: SqrtStreamMaker = (x, improve) => {
  const guesses: StreamCell<number> = consStream(1, () =>
    streamMap((guess) => improve(guess, x), guesses),
  );
  return guesses;
};

/** Louis's version from the statement: each tail maps over a fresh
 * recursive call, so no cell is shared between levels. */
export const sqrtStreamExternal: SqrtStreamMaker = (x, improve) =>
  consStream(1, () => streamMap((guess) => improve(guess, x), sqrtStreamExternal(x, improve)));

/** The book's `cons-stream` with the `memo-proc` optimization removed:
 * the tail is a plain lambda, recomputed on every force. */
const consUnmemoized = (head: number, tail: () => Stream<number>): StreamCell<number> => ({
  head,
  tail,
});

/** The book's `stream-map` over plain-lambda tails. */
const streamMapUnmemoized = (proc: (guess: number) => number, s: Stream<number>): Stream<number> =>
  s === null ? null : consUnmemoized(proc(s.head), () => streamMapUnmemoized(proc, s.tail()));

/** Alyssa's shape with plain-lambda tails: the binding is still local,
 * but every force rebuilds the mapped cells it walks through. */
export const sqrtStreamUnmemoizedLocal: SqrtStreamMaker = (x, improve) => {
  const guesses: StreamCell<number> = consUnmemoized(1, () =>
    streamMapUnmemoized((guess) => improve(guess, x), guesses),
  );
  return guesses;
};

/** Louis's shape with plain-lambda tails. */
export const sqrtStreamUnmemoizedExternal: SqrtStreamMaker = (x, improve) =>
  consUnmemoized(1, () =>
    streamMapUnmemoized((guess) => improve(guess, x), sqrtStreamUnmemoizedExternal(x, improve)),
  );

/** The guess at index n of a fresh stream, together with the number of
 * Newton steps computing it took. */
export interface SqrtCallCount {
  readonly guess: number;
  readonly calls: number;
}

/** Builds one fresh stream, walks it once to index n, and counts the
 * Newton steps the walk performed. */
export const improveCallsFor = (
  makeStream: SqrtStreamMaker,
  x: number,
  n: number,
): SqrtCallCount => {
  const counter: ImproveCounter = { calls: 0 };
  const improve = (guess: number, xx: number): number => {
    counter.calls += 1;
    return sqrtImprove(guess, xx);
  };
  return { guess: streamRef(makeStream(x, improve), n), calls: counter.calls };
};

/** The counts of walking one shared stream to m and then to n: with
 * memoized tails the extension beyond m costs only the new guesses;
 * without them the whole prefix is recomputed. */
export interface SqrtSharedStreamCount {
  readonly guessM: number;
  readonly guessN: number;
  readonly callsM: number;
  readonly callsN: number;
}

/** Builds one stream, refs it to m, then to n on the same object, and
 * reports the counter after each walk. */
export const improveCallsForSameStream = (
  makeStream: SqrtStreamMaker,
  x: number,
  m: number,
  n: number,
): SqrtSharedStreamCount => {
  const counter: ImproveCounter = { calls: 0 };
  const improve = (guess: number, xx: number): number => {
    counter.calls += 1;
    return sqrtImprove(guess, xx);
  };
  const guesses = makeStream(x, improve);
  const guessM = streamRef(guesses, m);
  const callsM = counter.calls;
  const guessN = streamRef(guesses, n);
  return { guessM, guessN, callsM, callsN: counter.calls };
};
