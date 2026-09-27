// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.2: line segments from points. A point is the host's pair of
 * numbers, a segment a pair of points, and every operation goes through
 * the constructors and selectors, so the two levels are separate
 * abstraction barriers. printPoint renders the same "(x,y)" the book's
 * procedure displays; this edition returns the string instead of printing.
 */
type Point = readonly [number, number];
type Segment = readonly [Point, Point];

/** Glues an x and a y coordinate into a point. */
export const makePoint = (x: number, y: number): Point => [x, y];

/** The x coordinate of a point. */
export const xPoint = (p: Point): number => p[0];

/** The y coordinate of a point. */
export const yPoint = (p: Point): number => p[1];

/** Glues a starting point and an ending point into a segment. */
export const makeSegment = (s: Point, e: Point): Segment => [s, e];

/** The starting point of a segment. */
export const startSegment = (s: Segment): Point => s[0];

/** The ending point of a segment. */
export const endSegment = (s: Segment): Point => s[1];

/** The midpoint of a segment: the averaged coordinates of its endpoints. */
export const midpointSegment = (s: Segment): Point => {
  const a = startSegment(s);
  const b = endSegment(s);
  return [(xPoint(a) + xPoint(b)) / 2, (yPoint(a) + yPoint(b)) / 2];
};

/** Renders a point the way the book's print-point displays it: "(x,y)". */
export const printPoint = (p: Point): string => `(${xPoint(p)},${yPoint(p)})`;
