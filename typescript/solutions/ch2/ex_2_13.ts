// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.13: the percentage tolerance of a product. For small
 * tolerances on positive numbers, the product's percentage tolerance is
 * approximately the sum of the factors': writing each factor as
 * c(1 +- t), the product is c1c2(1 +- (t1 + t2) +- higher-order terms),
 * and the dropped terms are of order t^2.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** Builds the interval centered at c with percentage tolerance p. */
export const makeCenterPercent = (c: number, p: number): Interval => {
  const half = (Math.abs(c) * p) / 100;
  return { lo: c - half, hi: c + half };
};

/** Multiplies through the four corner products and keeps the extreme ones. */
export const mulInterval = (x: Interval, y: Interval): Interval => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

/** The percentage tolerance of an interval. */
export const percent = (i: Interval): number =>
  (100 * (i.hi - i.lo)) / (2 * Math.abs((i.lo + i.hi) / 2));

/** The percentage tolerance of the product p * q. */
export const productPercent = (p: Interval, q: Interval): number => percent(mulInterval(p, q));
