// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { integralDelayed, type Stream, streamMap2 } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.79: the general second-order solver. The statement
 * generalizes `solve2nd` of exercise 3.78 to any second-order
 * equation `d^2y/dt^2 = f(dy/dt, y)`: instead of summing the two
 * scaled taps, the network feeds an arbitrary function of the two
 * state signals into the derivative integrator. The statement's
 * argument order is kept, so this edition's `f` receives the
 * derivative first and the position second, spelled `f(dy, y)`;
 * everything else is the two coupled `integralDelayed` loops of
 * exercise 3.78 with the sum replaced by the call to `f`.
 */

/** The statement's `f`: answers `y''` from `y'` and `y`, in the
 * statement's order (derivative first, position second). */
export type SecondDerivative = (dy: number, y: number) => number;

/** The book's generalized `solve2nd`: the stream of `y` values for
 * `y'' = f(y', y)` from `y(0) = y0` and `y'(0) = dy0`, integrated at
 * step `dt`. */
export const solveGeneral = (
  f: SecondDerivative,
  y0: number,
  dy0: number,
  dt: number,
): Stream<number> => {
  const y: Stream<number> = integralDelayed(() => dy, y0, dt);
  const dy: Stream<number> = integralDelayed(() => streamMap2((d, yy) => f(d, yy), dy, y), dy0, dt);
  return y;
};
