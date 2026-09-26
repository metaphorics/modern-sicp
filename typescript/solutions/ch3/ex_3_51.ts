// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type Stream,
  streamEnumerateInterval,
  streamMap,
  streamRef,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.51: `show` reveals when the delay computes. The book's
 * `show` prints its argument and returns it; wrapped in `stream-map`
 * over the interval 0 through 10, every element that gets computed
 * announces itself. This edition's transcript array stands in for what
 * the interpreter prints, so the answer is read off the transcript
 * after each evaluation: defining `x` computes only the first cell
 * (the car of the map is eager), the first ref walks 0 through 5, and
 * the second ref finds everything up to 5 memoized by the section's
 * `memo-proc` delay and computes only the two new cells.
 */

/** The book's `show`: renders its argument into the transcript and
 * returns the argument unchanged. */
export const show = <A>(x: A, transcript: string[]): A => {
  transcript.push(String(x));
  return x;
};

/** The book's `x`: show mapped over the interval 0 through 10. The
 * transcript collects the lines the interpreter would print as cells
 * get computed; the first line lands at definition time. */
export const makeX = (transcript: string[]): Stream<number> =>
  streamMap((n) => show(n, transcript), streamEnumerateInterval(0, 10));

/** The exercise's sequence as data: the transcript after the
 * definition, after `(stream-ref x 5)`, and after `(stream-ref x 7)`
 * on the same stream. The values the two refs answer, 5 and 7, ride
 * along. */
export interface ShowTranscript {
  afterDefine: string[];
  afterRef5: string[];
  afterRef7: string[];
  ref5: number;
  ref7: number;
}

/** Runs the book's three evaluations over one shared `x` and reports
 * the transcript after each. */
export const ex351 = (): ShowTranscript => {
  const lines: string[] = [];
  const x = makeX(lines);
  const afterDefine = [...lines];
  const ref5 = streamRef(x, 5);
  const afterRef5 = [...lines];
  const ref7 = streamRef(x, 7);
  return { afterDefine, afterRef5, afterRef7: [...lines], ref5, ref7 };
};

/** The counterfactual the timing rests on: a map built with plain
 * thunk tails, the delay of `(lambda () exp)` without `memo-proc`.
 * The first cell still computes at definition, but re-walking the
 * stream rebuilds every cell it passes, so the same second ref
 * announces the whole prefix again. */
export const makeUnmemoizedX = (transcript: string[]): Stream<number> => {
  const build = (n: number): Stream<number> => ({
    head: show(n, transcript),
    tail: () => (n >= 10 ? null : build(n + 1)),
  });
  return build(0);
};
