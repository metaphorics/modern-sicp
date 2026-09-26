// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  integers,
  type Stream,
  streamCdr,
  streamMap,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.71: numbers expressible as the sum of two cubes in more
 * than one way are sometimes called Ramanujan numbers. Ordered streams
 * of pairs give an elegant solution: generate the stream of pairs of
 * integers (i, j) weighted by the sum i^3 + j^3 (exercise 3.70), then
 * search the stream for two consecutive pairs with the same weight.
 * The first such number is 1729; the statement asks for the next five.
 */

/** One pair of integers. */
export type Pair = [number, number];

/** A weighting function over pairs, exercise 3.70's W(i, j). */
export type Weight = (pair: Pair) => number;

/** Exercise 3.70's `merge-weighted`, restated here because exercise
 * files do not import from each other: like `merge`, ordered by
 * weight, keeping both elements on equal weights (s1's head is served
 * and s2's head stays at its front for the next comparison). */
export const mergeWeighted = (s1: Stream<Pair>, s2: Stream<Pair>, weight: Weight): Stream<Pair> => {
  if (s1 === null) {
    return s2;
  }
  if (s2 === null) {
    return s1;
  }
  if (weight(s1.head) <= weight(s2.head)) {
    return consStream(s1.head, () => mergeWeighted(streamCdr(s1), s2, weight));
  }
  return consStream(s2.head, () => mergeWeighted(s1, streamCdr(s2), weight));
};

/** Exercise 3.70's `weighted-pairs`, restated: the module's `pairs`
 * shape with `merge-weighted` in place of the interleave. */
export const weightedPairs = (
  s: Stream<number>,
  t: Stream<number>,
  weight: Weight,
): Stream<Pair> => {
  if (s === null || t === null) {
    return null;
  }
  const s0 = s.head;
  const head: Pair = [s0, t.head];
  return consStream(head, () =>
    mergeWeighted(
      streamMap((x): Pair => [s0, x], streamCdr(t)),
      weightedPairs(streamCdr(s), streamCdr(t), weight),
      weight,
    ),
  );
};

/** The statement's weight: the sum of two cubes i^3 + j^3. */
const cubeWeight = ([i, j]: Pair): number => i ** 3 + j ** 3;

/** One Ramanujan hit: the number, then its two representations as
 * consecutive pairs of equal weight. */
export type RamanujanRun = [number, Pair, Pair];

/** The Ramanujan numbers: the weighted stream scanned for two
 * consecutive pairs with the same weight. */
export const ramanujanNumbers = (): Stream<RamanujanRun> => {
  const weighted = weightedPairs(integers, integers, cubeWeight);
  if (weighted === null) {
    return null;
  }
  const scan = (rest: Stream<Pair>, prev: Pair): Stream<RamanujanRun> => {
    if (rest === null) {
      return null;
    }
    const cur = rest.head;
    if (cubeWeight(cur) === cubeWeight(prev)) {
      return consStream([cubeWeight(cur), prev, cur], () => scan(streamCdr(rest), cur));
    }
    return scan(streamCdr(rest), cur);
  };
  return scan(streamCdr(weighted), weighted.head);
};
