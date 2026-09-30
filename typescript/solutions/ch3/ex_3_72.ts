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
 * Exercise 3.72: in a similar way to exercise 3.71, generate a stream
 * of all numbers that can be written as the sum of two squares in
 * three different ways, showing how they can be so written. The pairs
 * (i, j) with i <= j are ordered by the weight i^2 + j^2, and the scan
 * looks for three consecutive pairs of equal weight.
 */

/** One pair of integers. */
export type Pair = [number, number];

/** A weighting function over pairs, exercise 3.70's W(i, j). */
export type Weight = (pair: Pair) => number;

/** Exercise 3.70's `mergeWeighted`, restated here because exercise
 * files do not import from each other: like `merge`, ordered by
 * weight, keeping both elements on equal weights. */
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

/** Exercise 3.70's `weightedPairs`, restated: the module's `pairs`
 * shape with `mergeWeighted` in place of the interleave. */
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

/** The weight of exercise 3.72: the sum of two squares i^2 + j^2. */
const squareWeight = ([i, j]: Pair): number => i * i + j * j;

/** One hit: the number and its three representations as consecutive
 * pairs of equal weight. */
export type ThreeSquareRun = [number, Pair, Pair, Pair];

/** The numbers writable as a sum of two squares in three different
 * ways: the square-weighted stream scanned for three consecutive pairs
 * of equal weight, each hit reported with its three representations. */
export const threeSquareRepresentations = (): Stream<ThreeSquareRun> => {
  const weighted = weightedPairs(integers, integers, squareWeight);
  if (weighted === null) {
    return null;
  }
  const second = streamCdr(weighted);
  if (second === null) {
    return null;
  }
  const scan = (rest: Stream<Pair>, a: Pair, b: Pair): Stream<ThreeSquareRun> => {
    if (rest === null) {
      return null;
    }
    const c = rest.head;
    if (squareWeight(a) === squareWeight(b) && squareWeight(b) === squareWeight(c)) {
      return consStream([squareWeight(c), a, b, c], () => scan(streamCdr(rest), b, c));
    }
    return scan(streamCdr(rest), b, c);
  };
  return scan(streamCdr(second), weighted.head, second.head);
};
