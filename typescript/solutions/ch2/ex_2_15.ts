// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.15: is Eva right that par2 is "better"? Yes, for this
 * system: a formula that never repeats an uncertain variable gives
 * tighter bounds, because interval arithmetic cannot see that two
 * occurrences of the same name are the same quantity. par1 names r1 and
 * r2 twice each, par2 names each once, and on exercise 2.14's resistors
 * the tolerances come out 14.9 versus 5.0 percent.
 */
type Interval = { readonly lo: number; readonly hi: number };

const addInterval = (x: Interval, y: Interval): Interval => ({
  lo: x.lo + y.lo,
  hi: x.hi + y.hi,
});

const mulInterval = (x: Interval, y: Interval): Interval => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

const divInterval = (x: Interval, y: Interval): Interval =>
  mulInterval(x, { lo: 1 / y.hi, hi: 1 / y.lo });

const center = (i: Interval): number => (i.lo + i.hi) / 2;

const width = (i: Interval): number => (i.hi - i.lo) / 2;

/** The percentage tolerance of an interval. */
export const percent = (i: Interval): number => (100 * width(i)) / Math.abs(center(i));

/** Lem's direct formula, each resistor named twice. */
export const par1 = (r1: Interval, r2: Interval): Interval =>
  divInterval(mulInterval(r1, r2), addInterval(r1, r2));

/** Lem's rewritten formula, each resistor named once. */
export const par2 = (r1: Interval, r2: Interval): Interval => {
  const one: Interval = { lo: 1, hi: 1 };
  return divInterval(one, addInterval(divInterval(one, r1), divInterval(one, r2)));
};

/** True when par2's percentage tolerance is the tighter of the two. */
export const par2IsTighter = (r1: Interval, r2: Interval): boolean =>
  percent(par2(r1, r2)) < percent(par1(r1, r2));
