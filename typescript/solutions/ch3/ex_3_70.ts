// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  integers,
  type Stream,
  streamCdr,
  streamFilter,
  streamMap,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.70: generate streams whose pairs appear in a useful order
 * rather than the ad hoc interleaving order. A weighting function
 * W(i, j) says that (i1, j1) is less than (i2, j2) when
 * W(i1, j1) < W(i2, j2). `mergeWeighted` is like `merge` except that
 * it takes an additional argument `weight`, used to determine the
 * order of the merged stream; `weightedPairs` generalizes `pairs` to
 * two streams ordered by weight. The statement's two streams:
 *
 *   a. all pairs of positive integers (i, j) with i <= j, ordered by
 *      the sum i + j;
 *   b. all pairs (i, j) with i <= j, where neither i nor j is
 *      divisible by 2, 3, or 5, ordered by the sum 2i + 3j + 5ij.
 */

/** One pair of integers. */
export type Pair = [number, number];

/** A weighting function over pairs, the statement's W(i, j). */
export type Weight = (pair: Pair) => number;

/** The statement's `mergeWeighted`: like `merge`, ordered by weight.
 * On equal weights both elements are kept: s1's head is served and s2's
 * head stays at its front for the next comparison, so a run of equal
 * weights drains before anything heavier emerges. */
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

/** The statement's `weightedPairs`: the module's `pairs` shape with
 * `mergeWeighted` in place of the interleave. */
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

/** The statement's stream (a): the pairs (i, j) with i <= j, ordered
 * by the sum i + j. */
export const pairsBySum: Stream<Pair> = weightedPairs(integers, integers, ([i, j]) => i + j);

/** True when n is divisible by none of 2, 3, or 5, the statement's
 * membership test for stream (b). */
const notDivisibleByTwoThreeOrFive = (n: number): boolean =>
  n % 2 !== 0 && n % 3 !== 0 && n % 5 !== 0;

const integersNotDivisibleByTwoThreeOrFive = streamFilter(notDivisibleByTwoThreeOrFive, integers);

/** The statement's stream (b): pairs (i, j) with i <= j over integers
 * divisible by none of 2, 3, or 5, ordered by 2i + 3j + 5ij. */
export const pairsByWeightedSum: Stream<Pair> = weightedPairs(
  integersNotDivisibleByTwoThreeOrFive,
  integersNotDivisibleByTwoThreeOrFive,
  ([i, j]) => 2 * i + 3 * j + 5 * i * j,
);
