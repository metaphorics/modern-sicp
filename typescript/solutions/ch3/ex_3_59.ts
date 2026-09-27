// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  integersStartingFrom,
  type Stream,
  streamMap,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.59: power series as coefficient streams. The statement's
 * integral rule: the integral of a0 + a1*x + a2*x^2 + ... is
 * c + a0*x + (1/2)a1*x^2 + (1/3)a2*x^3 + ..., so `integrate-series`
 * divides the nth coefficient by n+1 and conses no constant. Since
 * d/dx e^x = e^x, the exp series is the constant e^0 = 1 followed by
 * the integral of itself. Since d/dx sin = cos and d/dx cos = -sin,
 * the sine series is the constant sin(0) = 0 followed by the integral
 * of the cosine series, and the cosine series is the constant cos(0)
 * = 1 followed by the integral of negative sine.
 */

/** The book's `integrate-series`: the non-constant coefficients of
 * the integral, a0, a1/2, a2/3, ..., with the constant left for the
 * caller to cons. */
export const integrateSeries = (s: Stream<number>): Stream<number> =>
  streamMap2((a, n) => a / n, s, integersStartingFrom(1));

/** Negation of a coefficient stream: the negative sine the cosine
 * derivation integrates. */
const negateSeries = (s: Stream<number>): Stream<number> => streamMap((a) => 0 - a, s);

/** The book's `exp-series`: 1 consed onto the integral of itself,
 * since e^x is its own derivative with e^0 = 1. */
export const expSeries: Stream<number> = consStream(1, () => integrateSeries(expSeries));

/** The book's `cosine-series`: constant term cos(0) = 1, then the
 * integral of -sin. */
export const cosineSeries: Stream<number> = consStream(1, () =>
  integrateSeries(negateSeries(sineSeries)),
);

/** The book's `sine-series`: constant term sin(0) = 0, then the
 * integral of cos. */
export const sineSeries: Stream<number> = consStream(0, () => integrateSeries(cosineSeries));
