// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  allInterleavings,
  type Process,
  runInterleaving,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.39: which of the five possibilities remain when the
 * square's computation runs under the serializer but its `set!` does
 * not, while the increment is serialized whole. The book's `(s (lambda
 * () (* x x)))` becomes a two-chunk process: one serialized chunk that
 * reads `x` twice and computes the product, then the plain write; the
 * increment is one serialized chunk. Three chunks, run in every order,
 * are the complete answer.
 */

/** The shared cell holding the book's `x`. */
export interface XCell {
  x: number;
}

/** Runs the exercise's two processes in every schedule and answers the
 * distinct final values of `x`, ascending. */
export const exercise39Outcomes = (): ReadonlyArray<number> => {
  const outcomes = new Set<number>();
  for (const order of allInterleavings([2, 1])) {
    const cell: XCell = { x: 10 };
    // P1: the serialized product computation, then the unserialized set!.
    const square: Process = (function* () {
      const a = cell.x;
      const b = cell.x;
      const product = a * b;
      yield;
      cell.x = product;
    })();
    // P2: the whole update inside the serializer, one chunk.
    const increment: Process =
      // biome-ignore lint/correctness/useYield: a serialized chunk is one scheduler step
      (function* () {
        const current = cell.x;
        cell.x = current + 1;
      })();
    runInterleaving([square, increment], order);
    outcomes.add(cell.x);
  }
  return [...outcomes].sort((a, b) => a - b);
};
