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
 * Exercise 3.60: the statement's `mulSeries` skeleton is
 * `consStream(⟨??⟩, () => addStreams(⟨??⟩, ⟨??⟩))`. Writing S1 = a0 + x*A
 * and S2 = b0 + x*B, the product is a0*S2 + x*(A*S2): the constant
 * term is a0*b0, and the tail is the product of the tail of s1 with
 * all of s2 plus the tail of s2 scaled by a0. The first blank after
 * `addStreams` is that recursive product; the second blank is the
 * scaled tail.
 */

/** The book's `mul-series` completed: a0*b0 consed onto the cross
 * terms, the tail-of-s1 by s2 product plus the tail of s2 scaled by
 * a0. */
export const mulSeries = (s1: Stream<number>, s2: Stream<number>): Stream<number> =>
  consStream(streamCar(s1) * streamCar(s2), () =>
    addStreams(mulSeries(streamCdr(s1), s2), scaleStream(streamCdr(s2), streamCar(s1))),
  );

/** The sine and cosine coefficient streams of Exercise 3.59, from the
 * derivative facts d/dx sin = cos and d/dx cos = -sin, restated here
 * because exercise files do not import from each other; the
 * statement's own check for this exercise squares both series. */
export const cosineSeries: Stream<number> = consStream(1, () =>
  integrateSeries(negateSeries(sineSeries)),
);

export const sineSeries: Stream<number> = consStream(0, () => integrateSeries(cosineSeries));

/** The exp coefficient stream of Exercise 3.59, from d/dx e^x = e^x,
 * restated for the exp(2x) = exp(x) * exp(x) check. */
export const expSeries: Stream<number> = consStream(1, () => integrateSeries(expSeries));

const negateSeries = (s: Stream<number>): Stream<number> => streamMap((a) => 0 - a, s);

/** The `integrateSeries` of Exercise 3.59: a0, a1/2, a2/3, .... */
const integrateSeries = (s: Stream<number>): Stream<number> =>
  streamMap2((a, n) => a / n, s, integersStartingFrom(1));
