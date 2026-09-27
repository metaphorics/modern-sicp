// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Process } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.40: all possible values of the concurrent square and cube.
 * Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_40.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.40 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The shared cell holding the book's `x`. */
export interface XCell {
  x: number;
}

/** The square process `x = (* x x)`: access, access, write. */
export function squareProcess(_cell: XCell): Process {
  throw new PendingSolution();
}

/** The cube process `x = (* x x x)`: access, access, access, write. */
export function cubeProcess(_cell: XCell): Process {
  throw new PendingSolution();
}

/** All distinct final values of `x` over every interleaving of the
 * unsynchronized square and cube, ascending. */
export function unsynchronizedOutcomes(): ReadonlyArray<number> {
  throw new PendingSolution();
}

/** All distinct final values when the two updates are serialized. */
export function serializedOutcomes(): ReadonlyArray<number> {
  throw new PendingSolution();
}
