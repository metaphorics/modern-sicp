// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.70: a weighting function W(i, j) says that (i1, j1) is
 * less than (i2, j2) when W(i1, j1) < W(i2, j2). Write
 * `merge-weighted`, like `merge` except that it takes an additional
 * argument `weight` used to determine the order of the merged stream,
 * and generalize `pairs` to `weighted-pairs`, ordered by weight. Use
 * them to generate (a) the pairs of positive integers (i, j) with
 * i <= j ordered by the sum i + j, and (b) the pairs (i, j) with
 * i <= j where neither i nor j is divisible by 2, 3, or 5, ordered by
 * 2i + 3j + 5ij. Pending scaffold; the solution and its rationale live
 * in solutions/ch3/ex_3_70.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.70 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One pair of integers. */
export type Pair = [number, number];

/** A weighting function over pairs, the statement's W(i, j). */
export type Weight = (pair: Pair) => number;

/** The statement's `merge-weighted`: like `merge`, ordered by weight,
 * keeping both elements on equal weights. */
export function mergeWeighted(_s1: Stream<Pair>, _s2: Stream<Pair>, _weight: Weight): Stream<Pair> {
  throw new PendingSolution();
}

/** The statement's `weighted-pairs`: the pairs of two streams ordered
 * by weight. */
export function weightedPairs(
  _s: Stream<number>,
  _t: Stream<number>,
  _weight: Weight,
): Stream<Pair> {
  throw new PendingSolution();
}

/** The statement's stream (a): pairs (i, j) with i <= j, by i + j. */
export function pairsBySum(): Stream<Pair> {
  throw new PendingSolution();
}

/** The statement's stream (b): pairs over integers with no divisor
 * 2, 3, or 5, by 2i + 3j + 5ij. */
export function pairsByWeightedSum(): Stream<Pair> {
  throw new PendingSolution();
}
