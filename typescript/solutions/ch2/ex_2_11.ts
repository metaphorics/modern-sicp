// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.11: multiplication in nine sign-tested cases. Each
 * interval is classified as positive, negative, or spanning zero, and
 * the pair of classes picks the two endpoint products that bound the
 * result - all four products only when both intervals span zero, which
 * is Ben's "only one of the nine". The non-spanning cases come from
 * treating a negative interval as the negation of a positive one and
 * tracking the sign flips; the exhaustive grid test checks every case
 * against the naive four-product multiplication.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** The sign class of an interval: positive, negative, or spanning zero. */
export type Sign = "positive" | "negative" | "span";

/** Classifies an interval by the signs of its endpoints. */
export const signOf = (x: Interval): Sign => {
  if (x.lo > 0) {
    return "positive";
  }
  if (x.hi < 0) {
    return "negative";
  }
  return "span";
};

/** Multiplies through the naive four corner products. */
const mulIntervalNaive = (x: Interval, y: Interval): Interval => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

/** Multiplies through the one of nine cases the two sign classes select. */
export const mulIntervalFast = (x: Interval, y: Interval): Interval => {
  const sx = signOf(x);
  const sy = signOf(y);
  if (sx === "positive" && sy === "positive") {
    return { lo: x.lo * y.lo, hi: x.hi * y.hi };
  }
  if (sx === "positive" && sy === "negative") {
    return { lo: x.hi * y.lo, hi: x.lo * y.hi };
  }
  if (sx === "negative" && sy === "positive") {
    return { lo: x.lo * y.hi, hi: x.hi * y.lo };
  }
  if (sx === "negative" && sy === "negative") {
    return { lo: x.hi * y.hi, hi: x.lo * y.lo };
  }
  if (sx === "positive" && sy === "span") {
    return { lo: x.hi * y.lo, hi: x.hi * y.hi };
  }
  if (sx === "span" && sy === "positive") {
    return { lo: x.lo * y.hi, hi: x.hi * y.hi };
  }
  if (sx === "negative" && sy === "span") {
    return { lo: x.lo * y.hi, hi: x.lo * y.lo };
  }
  if (sx === "span" && sy === "negative") {
    return { lo: x.hi * y.lo, hi: x.lo * y.lo };
  }
  return mulIntervalNaive(x, y);
};
