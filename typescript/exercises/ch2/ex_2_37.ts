// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.37: matrix operations as sequence operations. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.37 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The sum of the entrywise products of `v` and `w`. */
export function dotProduct(_v: List<number>, _w: List<number>): number {
  throw new PendingSolution();
}

/** Each row of `m` dotted with the vector `v`. */
export function matrixTimesVector(_m: List<List<number>>, _v: List<number>): List<number> {
  throw new PendingSolution();
}

/** The columns of `m` become the rows of the result. */
export function transpose(_m: List<List<number>>): List<List<number>> {
  throw new PendingSolution();
}

/** Each entry is the dot product of a row of `m` with a column of `n`. */
export function matrixTimesMatrix(
  _m: List<List<number>>,
  _n: List<List<number>>,
): List<List<number>> {
  throw new PendingSolution();
}
