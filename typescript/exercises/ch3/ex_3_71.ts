// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.71: numbers that can be expressed as the sum of two cubes
 * in more than one way are sometimes called Ramanujan numbers. Ordered
 * streams of pairs provide an elegant solution: generate the stream of
 * pairs of integers (i, j) weighted by the sum i^3 + j^3, then search
 * the stream for two consecutive pairs with the same weight. The first
 * such number is 1729; what are the next five? Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_71.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.71 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One pair of integers. */
export type Pair = [number, number];

/** One Ramanujan hit: the number, then its two representations as
 * consecutive pairs of equal weight. */
export type RamanujanRun = [number, Pair, Pair];

/** The Ramanujan numbers: the cube-weighted stream of pairs scanned
 * for two consecutive pairs with the same weight. */
export function ramanujanNumbers(): Stream<RamanujanRun> {
  throw new PendingSolution();
}
