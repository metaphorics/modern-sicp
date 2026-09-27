// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.9: the width algebra. The width of a sum (or difference) is
 * a function of the argument widths alone - the bounds move pairwise, so
 * the halves add - while for multiplication the width of the product
 * also depends on the factors' centers, which two products with
 * equal-width but different-center factors demonstrate.
 *
 * This edition's invariant decision, which makes `width` well defined in
 * the first place: an interval is only constructed through a
 * constructor that enforces `lo <= hi` (the section module's
 * `makeInterval`, reporting `UnorderedBounds` otherwise). The decision is
 * settled once, at the constructor, not re-argued at every call site.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** Adds two intervals bound-wise. */
export const addInterval = (x: Interval, y: Interval): Interval => ({
  lo: x.lo + y.lo,
  hi: x.hi + y.hi,
});

/** Subtracts y from x with the bounds crossed. */
export const subInterval = (x: Interval, y: Interval): Interval => ({
  lo: x.lo - y.hi,
  hi: x.hi - y.lo,
});

/** Multiplies through the four corner products and keeps the extreme ones. */
export const mulInterval = (x: Interval, y: Interval): Interval => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

/** Divides x by y through the reciprocal, the section's pre-2.10 spelling. */
export const divInterval = (x: Interval, y: Interval): Interval =>
  mulInterval(x, { lo: 1 / y.hi, hi: 1 / y.lo });

/** The width of an interval: half the difference of its bounds. */
export const width = (i: Interval): number => (i.hi - i.lo) / 2;

/** True when the width of x + y equals width(x) + width(y) exactly. */
export const addWidthLaw = (x: Interval, y: Interval): boolean =>
  width(addInterval(x, y)) === width(x) + width(y);
