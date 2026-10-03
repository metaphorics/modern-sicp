// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.46: the picture language's vectors, with a constructor
 * `makeVect` and selectors `xcor-vect` and `ycor-vect`, plus the
 * `add-vect`, `sub-vect` and `scale-vect` operations the frame
 * coordinate map composes. This edition represents a vector as a pair
 * of coordinates: a readonly two-element tuple.
 */
export type Vect2 = readonly [number, number];

/** The book's `makeVect`: bundles two coordinates into a vector. */
export const makeVect = (x: number, y: number): Vect2 => [x, y];

/** The book's `xcor-vect`: the first coordinate. */
export const xCorVect = (v: Vect2): number => v[0];

/** The book's `ycor-vect`: the second coordinate. */
export const yCorVect = (v: Vect2): number => v[1];

/** The book's `add-vect`: the componentwise sum. */
export const addVect2 = (v: Vect2, w: Vect2): Vect2 => [v[0] + w[0], v[1] + w[1]];

/** The book's `sub-vect`: the componentwise difference. */
export const subVect2 = (v: Vect2, w: Vect2): Vect2 => [v[0] - w[0], v[1] - w[1]];

/** The book's `scale-vect`: each component times the factor. */
export const scaleVect2 = (s: number, v: Vect2): Vect2 => [s * v[0], s * v[1]];
