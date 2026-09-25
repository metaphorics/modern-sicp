// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.3: two rectangle representations behind one abstraction
 * barrier. Both constructors build the same Rectangle union, and the
 * perimeter and area procedures work through the width and height
 * selectors, never through a representation directly.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.3 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A rectangle as two opposite corners, or as a corner plus width and height. */
export type Rectangle =
  | {
      readonly _tag: "Corners";
      readonly a: readonly [number, number];
      readonly b: readonly [number, number];
    }
  | {
      readonly _tag: "BaseHeight";
      readonly corner: readonly [number, number];
      readonly w: number;
      readonly h: number;
    };

/** Builds the corner-pair representation from two opposite corners. */
export function makeRectangleCorners(
  _a: readonly [number, number],
  _b: readonly [number, number],
): Rectangle {
  throw new PendingSolution();
}

/** Builds the base-and-height representation from a corner and two extents. */
export function makeRectangleBaseHeight(
  _corner: readonly [number, number],
  _w: number,
  _h: number,
): Rectangle {
  throw new PendingSolution();
}

/** The width of the rectangle, above either representation. */
export function widthOf(_r: Rectangle): number {
  throw new PendingSolution();
}

/** The height of the rectangle, above either representation. */
export function heightOf(_r: Rectangle): number {
  throw new PendingSolution();
}

/** The perimeter, 2(w + h), working over either representation. */
export function perimeter(_r: Rectangle): number {
  throw new PendingSolution();
}

/** The area, w * h, working over either representation. */
export function area(_r: Rectangle): number {
  throw new PendingSolution();
}
