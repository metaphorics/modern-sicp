// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  addStreams,
  consStream,
  integersStartingFrom,
  type Stream,
  scaleStream,
  streamCar,
  streamCdr,
  streamMap,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.61: the statement solves S*X = 1 by writing S = 1 + S_R,
 * the part after the constant term, and rearranging: (1 + S_R)*X = 1,
 * X + S_R*X = 1, X = 1 - S_R*X. So the reciprocal series X is the
 * constant 1 followed by the negation of the product of S_R with X
 * itself, a self-referential stream that unfolds one coefficient per
 * level.
 */

/** The book's `mul-series` from Exercise 3.60, restated here because
 * exercise files do not import from each other; the derivation needs
 * it to express X = 1 - S_R * X. */
export const mulSeries = (s1: Stream<number>, s2: Stream<number>): Stream<number> =>
  consStream(streamCar(s1) * streamCar(s2), () =>
    addStreams(mulSeries(streamCdr(s1), s2), scaleStream(streamCdr(s2), streamCar(s1))),
  );

/** The exp coefficient stream of Exercise 3.59 (constant term 1, so a
 * legal unit series to invert), restated for the checks. */
export const expSeries: Stream<number> = consStream(1, () =>
  streamMap2((a, n) => a / n, expSeries, integersStartingFrom(1)),
);

/** Negation of a coefficient stream: the minus sign of
 * X = 1 - S_R * X. */
const negateSeries = (s: Stream<number>): Stream<number> => streamMap((a) => 0 - a, s);

/** The book's `invert-unit-series`: the series X with constant term 1
 * whose remainder is the negation of S_R (the part of s after the
 * constant term) times X itself. */
export const invertUnitSeries = (s: Stream<number>): Stream<number> =>
  consStream(1, () => negateSeries(mulSeries(streamCdr(s), invertUnitSeries(s))));
