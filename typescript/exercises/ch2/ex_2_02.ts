// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.2: line segments built from points. A point is a pair of
 * numbers, a segment a pair of points, and the midpoint averages the
 * coordinates of the endpoints.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.2 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Glues an x and a y coordinate into a point. */
export function makePoint(_x: number, _y: number): readonly [number, number] {
  throw new PendingSolution();
}

/** The x coordinate of a point. */
export function xPoint(_p: readonly [number, number]): number {
  throw new PendingSolution();
}

/** The y coordinate of a point. */
export function yPoint(_p: readonly [number, number]): number {
  throw new PendingSolution();
}

/** Glues a starting point and an ending point into a segment. */
export function makeSegment(
  _s: readonly [number, number],
  _e: readonly [number, number],
): readonly [readonly [number, number], readonly [number, number]] {
  throw new PendingSolution();
}

/** The starting point of a segment. */
export function startSegment(
  _s: readonly [readonly [number, number], readonly [number, number]],
): readonly [number, number] {
  throw new PendingSolution();
}

/** The ending point of a segment. */
export function endSegment(
  _s: readonly [readonly [number, number], readonly [number, number]],
): readonly [number, number] {
  throw new PendingSolution();
}

/** The midpoint of a segment: the averaged coordinates of its endpoints. */
export function midpointSegment(
  _s: readonly [readonly [number, number], readonly [number, number]],
): readonly [number, number] {
  throw new PendingSolution();
}

/** Renders a point the way the book's print-point does: "(x,y)". */
export function printPoint(_p: readonly [number, number]): string {
  throw new PendingSolution();
}
