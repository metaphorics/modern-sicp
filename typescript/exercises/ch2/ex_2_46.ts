// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.46: a two-dimensional vector abstraction for the picture
 * language, represented as a pair.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.46 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A vector in the plane: the pair (x, y). */
export type Vect2 = readonly [number, number];

/** The book's make-vect: bundles two coordinates into a vector. */
export function makeVect(_x: number, _y: number): Vect2 {
  throw new PendingSolution();
}

/** The book's xcor-vect: the first coordinate. */
export function xCorVect(_v: Vect2): number {
  throw new PendingSolution();
}

/** The second coordinate selector. */
export function yCorVect(_v: Vect2): number {
  throw new PendingSolution();
}

/** The book's add-vect: the componentwise sum. */
export function addVect2(_v: Vect2, _w: Vect2): Vect2 {
  throw new PendingSolution();
}

/** The book's sub-vect: the componentwise difference. */
export function subVect2(_v: Vect2, _w: Vect2): Vect2 {
  throw new PendingSolution();
}

/** The book's scale-vect: each component times the factor. */
export function scaleVect2(_s: number, _v: Vect2): Vect2 {
  throw new PendingSolution();
}
