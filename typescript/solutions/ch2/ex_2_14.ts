// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.14: Lem is right. The investigation computes A / A and the
 * two parallel-resistance formulas on intervals whose width is a small
 * percentage of the center, and reads the results in center-percent
 * form. Interval arithmetic treats every textual occurrence of a
 * variable as an independent uncertain quantity, so A / A is not the
 * point [1, 1], par1 (each resistor twice) is far wider than par2 (each
 * resistor once), and the numbers show it.
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

/** The percentage tolerance of an interval, the center-percent reading. */
export const percent = (i: Interval): number => (100 * width(i)) / Math.abs(center(i));

/** Lem's direct formula: (r1 * r2) / (r1 + r2). */
export const par1 = (r1: Interval, r2: Interval): Interval =>
  divInterval(mulInterval(r1, r2), addInterval(r1, r2));

/** Lem's rewritten formula: 1 / (1/r1 + 1/r2), each resistor once. */
export const par2 = (r1: Interval, r2: Interval): Interval => {
  const one: Interval = { lo: 1, hi: 1 };
  return divInterval(one, addInterval(divInterval(one, r1), divInterval(one, r2)));
};

/** The two resistors of the investigation: 10 and 20 ohms, each +- 5%. */
export const resistorA: Interval = { lo: 9.5, hi: 10.5 };

/** The second resistor of the investigation: 20 ohms +- 5%. */
export const resistorB: Interval = { lo: 19, hi: 21 };

/** The computed A / A, which is not the exact point [1, 1]. */
export const aOverA = (a: Interval): Interval => divInterval(a, a);

/** The computed A / B for the investigation's pair. */
export const aOverB = (a: Interval, b: Interval): Interval => divInterval(a, b);
