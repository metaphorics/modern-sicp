// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  allInterleavings,
  type Process,
  runInterleaving,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.40: all possible values of the concurrent square and cube.
 * The square is the book's three steps (two accesses of `x`, then the
 * write); the cube is four steps (three accesses, then the write). The
 * unsynchronized set comes from running all 35 interleavings of the two
 * processes; the serialized set runs the same mutation with each
 * process's whole update as one chunk. Both sets are produced by
 * execution, under the module's step scheduler.
 */

/** The shared cell holding the book's `x`. */
export interface XCell {
  x: number;
}

/** The square process `x = (* x x)`: access, access, write. */
export const squareProcess = (cell: XCell): Process =>
  (function* () {
    const a = cell.x;
    yield;
    const b = cell.x;
    yield;
    cell.x = a * b;
  })();

/** The cube process `x = (* x x x)`: access, access, access, write. */
export const cubeProcess = (cell: XCell): Process =>
  (function* () {
    const a = cell.x;
    yield;
    const b = cell.x;
    yield;
    const c = cell.x;
    yield;
    cell.x = a * b * c;
  })();

/** All distinct final values of `x` over every interleaving of the
 * unsynchronized square and cube, ascending. */
export const unsynchronizedOutcomes = (): ReadonlyArray<number> => {
  const outcomes = new Set<number>();
  for (const order of allInterleavings([3, 4])) {
    const cell: XCell = { x: 10 };
    runInterleaving([squareProcess(cell), cubeProcess(cell)], order);
    outcomes.add(cell.x);
  }
  return [...outcomes].sort((a, b) => a - b);
};

/** All distinct final values when the two updates are serialized: each
 * process's whole update is one chunk, so the two chunks run in one of
 * two orders. */
export const serializedOutcomes = (): ReadonlyArray<number> => {
  const outcomes = new Set<number>();
  for (const order of allInterleavings([1, 1])) {
    const cell: XCell = { x: 10 };
    const square: Process =
      // biome-ignore lint/correctness/useYield: a serialized chunk is one scheduler step
      (function* () {
        cell.x = cell.x * cell.x;
      })();
    const cube: Process =
      // biome-ignore lint/correctness/useYield: a serialized chunk is one scheduler step
      (function* () {
        cell.x = cell.x * cell.x * cell.x;
      })();
    runInterleaving([square, cube], order);
    outcomes.add(cell.x);
  }
  return [...outcomes].sort((a, b) => a - b);
};
