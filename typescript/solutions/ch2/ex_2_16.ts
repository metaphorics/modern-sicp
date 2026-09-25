// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.16: why equivalent algebraic expressions give different
 * answers, and whether the shortcoming can be removed. The smallest
 * instance is x - x: the package subtracts bound-wise as if the two
 * occurrences were independent quantities, so a 10-ohm 5-percent
 * resistor minus itself is [-1, 1], not the exact [0, 0] algebra
 * gives. No package built from independent endpoint bookkeeping can fix
 * this: it never sees the expression, only pairs of intervals, so it
 * cannot know that its two arguments are the same number. A fix would
 * have to carry each result's functional dependence on its inputs - in
 * the limit, to compute with the expression's exact function form and
 * evaluate it at the corners, which is the "very difficult" the book
 * warns of.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** Subtracts y from x bound-wise, treating the two as independent. */
export const subInterval = (x: Interval, y: Interval): Interval => ({
  lo: x.lo - y.hi,
  hi: x.hi - y.lo,
});

/** The package's answer for x - x on one interval: not the exact [0, 0]. */
export const xMinusX = (x: Interval): Interval => subInterval(x, x);
