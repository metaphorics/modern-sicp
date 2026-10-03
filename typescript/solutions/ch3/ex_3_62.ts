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
 * Exercise 3.62: the statement builds `divSeries` from the two prior
 * exercises: dividing by s2 means multiplying s1 by the reciprocal of
 * s2, and the reciprocal is invert-unit-series, legal exactly when
 * s2's constant term is nonzero (otherwise the inversion would divide
 * by zero, so divSeries signals an error). The requested use is the
 * tangent series, sin over cos, whose constant term is cos(0) = 1,
 * hence legal.
 */

/** The book's `mul-series` from Exercise 3.60, restated here because
 * exercise files do not import from each other. */
export const mulSeries = (s1: Stream<number>, s2: Stream<number>): Stream<number> =>
  consStream(streamCar(s1) * streamCar(s2), () =>
    addStreams(mulSeries(streamCdr(s1), s2), scaleStream(streamCdr(s2), streamCar(s1))),
  );

/** The book's `invert-unit-series` from Exercise 3.61: the reciprocal
 * of a unit series, X = 1 - S_R * X. */
export const invertUnitSeries = (s: Stream<number>): Stream<number> =>
  consStream(1, () => negateSeries(mulSeries(streamCdr(s), invertUnitSeries(s))));

/** The sine and cosine coefficient streams of Exercise 3.59, from the
 * derivative facts d/dx sin = cos and d/dx cos = -sin, restated here
 * because exercise files do not import from each other. */
export const cosineSeries: Stream<number> = consStream(1, () =>
  integrateSeries(negateSeries(sineSeries)),
);

export const sineSeries: Stream<number> = consStream(0, () => integrateSeries(cosineSeries));

const negateSeries = (s: Stream<number>): Stream<number> => streamMap((a) => 0 - a, s);

/** The `integrateSeries` of Exercise 3.59: a0, a1/2, a2/3, .... */
const integrateSeries = (s: Stream<number>): Stream<number> =>
  streamMap2((a, n) => a / n, s, integersStartingFrom(1));

/** The book's `divSeries`: s1/s2 as the mul-series of s1 with the
 * inverted s2. Refuses a denominator whose constant term is 0,
 * throwing an `Error` carrying the message "div-series: the
 * denominator has a zero constant term". */
export const divSeries = (s1: Stream<number>, s2: Stream<number>): Stream<number> => {
  if (streamCar(s2) === 0) {
    throw new Error("div-series: the denominator has a zero constant term");
  }
  return mulSeries(s1, invertUnitSeries(s2));
};

/** The book's tangent: sin divided by cos, the statement's requested
 * use of divSeries. */
export const tangent: Stream<number> = divSeries(sineSeries, cosineSeries);
