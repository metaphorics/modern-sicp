// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.72: in a similar way to exercise 3.71, generate a stream
 * of all numbers that can be written as the sum of two squares in
 * three different ways (showing how they can be so written). Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_72.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.72 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One pair of integers. */
export type Pair = [number, number];

/** One hit: the number and its three representations as consecutive
 * pairs of equal weight. */
export type ThreeSquareRun = [number, Pair, Pair, Pair];

/** The numbers writable as a sum of two squares in three different
 * ways, each reported with its three representations. */
export function threeSquareRepresentations(): Stream<ThreeSquareRun> {
  throw new PendingSolution();
}
