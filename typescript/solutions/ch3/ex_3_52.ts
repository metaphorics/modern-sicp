// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Ref } from "effect";

import {
  type Delayed,
  memoProc,
  type Stream,
  streamCdr,
  streamEnumerateInterval,
  streamRef,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.52: `accum` traces assignment plus laziness. The book's
 * `sum` is a mutable cell, so the edition spells it as an `Effect.Ref`
 * and `accum` as an effect that adds and stores; `Effect.suspend`
 * defers each addition to the moment a cell is actually built. The
 * transcript is the exercise's sequence over the interval 1 through
 * 20: define `seq` (accum mapped over the interval), `y` (the even
 * elements), `z` (the multiples of five), then `(stream-ref y 7)` and
 * `(display-stream z)`, reading the sum after each step. The
 * exercise's last question, whether a plain-thunk delay without
 * `memo-proc` would answer differently, is answered by running the
 * same transcript with the memoization switched off.
 */

/** The book's `accum` over the shared `sum` cell: adds `x` and answers
 * the new sum. */
export const makeAccum = (sum: Ref.Ref<number>): ((x: number) => Effect.Effect<number>) => {
  const accum = (x: number): Effect.Effect<number> =>
    Effect.gen(function* () {
      const current = yield* Ref.get(sum);
      return yield* Ref.setAndGet(sum, current + x);
    });
  return accum;
};

/** Runs the book's `(accum x)` now: suspend defers the mutation to
 * this exact call. */
export const runAccum = (accum: (x: number) => Effect.Effect<number>, x: number): number =>
  Effect.runSync(Effect.suspend(() => accum(x)));

/** The edition's `delay` with a switch: the memoized `memo-proc` delay
 * the section implements, or the plain thunk the exercise's last
 * question asks about. */
export const delaySwitch = <A>(thunk: () => A, memoized: boolean): Delayed<A> =>
  memoized ? memoProc(thunk) : thunk;

/** The book's one-argument stream-map under the switch. Each cell's
 * head computes when the cell is built, so the accumulator runs once
 * per position when memoized and once per visit when not. */
export const mapSwitch = <A, B>(proc: (a: A) => B, s: Stream<A>, memoized: boolean): Stream<B> => {
  if (s === null) {
    return null;
  }
  const head = proc(s.head);
  return { head, tail: delaySwitch(() => mapSwitch(proc, streamCdr(s), memoized), memoized) };
};

/** The book's stream-filter under the switch. */
export const filterSwitch = <A>(
  pred: (a: A) => boolean,
  s: Stream<A>,
  memoized: boolean,
): Stream<A> => {
  if (s === null) {
    return null;
  }
  if (pred(s.head)) {
    const head = s.head;
    return { head, tail: delaySwitch(() => filterSwitch(pred, streamCdr(s), memoized), memoized) };
  }
  return filterSwitch(pred, streamCdr(s), memoized);
};

/** One step of the transcript's readings. */
export interface AccumReadings {
  sumAfterSeq: number;
  sumAfterY: number;
  sumAfterZ: number;
  refAnswer: number;
  sumAfterRef: number;
  zDisplayed: number[];
  sumAfterDisplay: number;
}

/** Runs the book's sequence of definitions and reads the sum after
 * each, under the given delay discipline. The boolean selects between
 * the two named entry points below. */
const runTranscript = (memoized: boolean): Effect.Effect<AccumReadings> =>
  Effect.gen(function* () {
    const sum = yield* Ref.make(0);
    const accum = makeAccum(sum);
    const seq = mapSwitch((x) => runAccum(accum, x), streamEnumerateInterval(1, 20), memoized);
    const sumAfterSeq = yield* Ref.get(sum);
    const y = filterSwitch((x) => x % 2 === 0, seq, memoized);
    const sumAfterY = yield* Ref.get(sum);
    const z = filterSwitch((x) => x % 5 === 0, seq, memoized);
    const sumAfterZ = yield* Ref.get(sum);
    const refAnswer = streamRef(y, 7);
    const sumAfterRef = yield* Ref.get(sum);
    const zDisplayed: number[] = [];
    let rest = z;
    while (rest !== null) {
      zDisplayed.push(rest.head);
      rest = streamCdr(rest);
    }
    const sumAfterDisplay = yield* Ref.get(sum);
    return {
      sumAfterSeq,
      sumAfterY,
      sumAfterZ,
      refAnswer,
      sumAfterRef,
      zDisplayed,
      sumAfterDisplay,
    };
  });

/** The memoized run: the book's answers under the section's
 * `memo-proc` delay. */
export const memoizedRun = (): Effect.Effect<AccumReadings> => runTranscript(true);

/** The plain-thunk run: the exercise's counterfactual. */
export const unmemoizedRun = (): Effect.Effect<AccumReadings> => runTranscript(false);
