// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  mapSuccessivePairs,
  monteCarloStream,
  randomNumbers,
  type Stream,
  streamMap,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.82: Monte Carlo integration as streams, redoing exercise
 * 3.5. Exercise 3.5 asked for `estimate-integral`, a procedure that
 * estimates the area of the region a predicate `P` holds over inside
 * a rectangle by choosing random points in the rectangle and running
 * the pass/fail outcomes through the Monte Carlo method. The stream
 * version keeps the experiment but drops the trial count: the stream
 * of random points flows through the section's `monte-carlo` and
 * answers a stream of successively better estimates, each one the
 * passing fraction times the rectangle's area, so looking farther
 * into the stream means more trials, as with the section's `pi`.
 */

/** The xorshift32 range: `rand-update` answers the full uint32 span,
 * so dividing a state by this maps it onto [0, 1]. */
const randMax = 4294967295;

/** One experiment point: two random states mapped uniformly onto the
 * rectangle [x1, x2] x [y1, y2]. */
export interface Point {
  readonly x: number;
  readonly y: number;
}

/** The book's "random points in the rectangle": the stream of points
 * drawn from the module's `randomNumbers`, two generator states per
 * point, mapped onto the given rectangle. */
export const randomPointsIn = (x1: number, x2: number, y1: number, y2: number): Stream<Point> =>
  mapSuccessivePairs(
    (r1, r2) => ({
      x: x1 + (r1 / randMax) * (x2 - x1),
      y: y1 + (r2 / randMax) * (y2 - y1),
    }),
    randomNumbers,
  );

/** The book's `estimate-integral`, stream edition: the stream of
 * Monte Carlo estimates of the area where `pred` holds inside the
 * rectangle, estimate `n` taken over the first `n + 1` trials, each
 * estimate the passing fraction scaled by the rectangle's area. */
export const estimateIntegral = (
  pred: (x: number, y: number) => boolean,
  x1: number,
  x2: number,
  y1: number,
  y2: number,
): Stream<number> => {
  const experiments: Stream<boolean> = streamMap(
    (p: Point) => pred(p.x, p.y),
    randomPointsIn(x1, x2, y1, y2),
  );
  const width = x2 - x1;
  const height = y2 - y1;
  return streamMap((p) => p * width * height, monteCarloStream(experiments, 0, 0));
};

/** Exercise 3.5's trial run: points in the square [-1, 1] x [-1, 1],
 * the region the inside of the unit circle, so the estimates home in
 * on the area of the circle, pi. */
export const unitCircleEstimates: Stream<number> = estimateIntegral(
  (x, y) => x * x + y * y <= 1,
  -1,
  1,
  -1,
  1,
);
