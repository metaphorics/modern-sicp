// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { integralDelayed, type Stream, streamMap2 } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.78: `solve-2nd`, the second-order feedback loop. The
 * statement's equation is `d^2y/dt^2 - a dy/dt - b y = 0`, that is
 * `y'' = a y' + b y`: the second derivative depends on both `y` and
 * its first derivative, and both of those are themselves determined
 * by integrating the second derivative, so the network of Figure 3.35
 * is two `integral` boxes in series with two scaled taps (`a` on the
 * derivative, `b` on the position) summed back into the input. This
 * edition spells the delayed integral `integralDelayed`, and its
 * thunked integrand is what lets the two `const` bindings below
 * refer to each other before either is initialized.
 */

/** The book's `solve-2nd`: the stream of successive values of `y`
 * for `y'' = a y' + b y`, from `y(0) = y0` and `y'(0) = dy0`,
 * integrated at step `dt`. The position loop integrates the
 * derivative loop; the derivative loop integrates `a y' + b y` read
 * off the two loops. */
export const solve2nd = (
  a: number,
  b: number,
  y0: number,
  dy0: number,
  dt: number,
): Stream<number> => {
  const y: Stream<number> = integralDelayed(() => dy, y0, dt);
  const dy: Stream<number> = integralDelayed(
    () => streamMap2((yy, d) => a * d + b * yy, y, dy),
    dy0,
    dt,
  );
  return y;
};
