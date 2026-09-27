// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.3: two rectangle representations behind one abstraction
 * barrier. The two constructors build different concrete shapes, but
 * `widthOf` and `heightOf` are the only thing `perimeter` and `area`
 * consume, so the same two procedures work over either representation.
 */
export type Point = readonly [number, number];

/** Builds the point the corner representations are expressed in. */
export const makePoint = (x: number, y: number): Point => [x, y];

/** A rectangle as two opposite corners, or as a corner plus width and height. */
export type Rectangle =
  | { readonly _tag: "Corners"; readonly a: Point; readonly b: Point }
  | { readonly _tag: "BaseHeight"; readonly corner: Point; readonly w: number; readonly h: number };

/** Builds the corner-pair representation from two opposite corners. */
export const makeRectangleCorners = (a: Point, b: Point): Rectangle => ({
  _tag: "Corners",
  a,
  b,
});

/** Builds the base-and-height representation from a corner and two extents. */
export const makeRectangleBaseHeight = (corner: Point, w: number, h: number): Rectangle => ({
  _tag: "BaseHeight",
  corner,
  w,
  h,
});

/** The width of the rectangle, above either representation. */
export const widthOf = (r: Rectangle): number => {
  switch (r._tag) {
    case "Corners":
      return Math.abs(r.b[0] - r.a[0]);
    case "BaseHeight":
      return r.w;
  }
};

/** The height of the rectangle, above either representation. */
export const heightOf = (r: Rectangle): number => {
  switch (r._tag) {
    case "Corners":
      return Math.abs(r.b[1] - r.a[1]);
    case "BaseHeight":
      return r.h;
  }
};

/** The perimeter, 2(w + h), working over either representation. */
export const perimeter = (r: Rectangle): number => 2 * (widthOf(r) + heightOf(r));

/** The area, w * h, working over either representation. */
export const area = (r: Rectangle): number => widthOf(r) * heightOf(r);
