// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  integers,
  interleave,
  pairs,
  type Stream,
  streamCdr,
  streamMap,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.69: write a procedure `triples` that takes three infinite
 * streams S, T, and U and produces the stream of triples (Si, Tj, Uk)
 * such that i <= j <= k. Use `triples` to generate the stream of all
 * Pythagorean triples of positive integers, the triples (i, j, k)
 * such that i <= j and i^2 + j^2 = k^2.
 *
 * The head triple is (S0, T0, U0). The rest interleaves the first-row
 * triples, the exercise 3.67 shape of the (T, U) plane prefixed by S0,
 * with the recursive triples over all three cdrs.
 */

/** One triple of integers. */
export type Triple = [number, number, number];

/** The statement's `triples`: (Si, Tj, Uk) with i <= j <= k. */
export const triples = (
  s: Stream<number>,
  t: Stream<number>,
  u: Stream<number>,
): Stream<Triple> => {
  if (s === null || t === null || u === null) {
    return null;
  }
  const s0 = s.head;
  const head: Triple = [s0, t.head, u.head];
  return consStream(head, () =>
    interleave(
      streamMap((p): Triple => [s0, p[0], p[1]], pairs(streamCdr(t), streamCdr(u))),
      triples(streamCdr(s), streamCdr(t), streamCdr(u)),
    ),
  );
};

/** The statement's stream-filter, spelled to skip iteratively: the
 * module's streamFilter recurses once per skipped element, and the
 * runs between Pythagorean triples are far too long for the call
 * stack. The skip is a loop, each hit's tail re-enters fresh, and the
 * filtered element order is the module filter's. */
const filterTriples = (s: Stream<Triple>, pred: (triple: Triple) => boolean): Stream<Triple> => {
  let rest = s;
  while (rest !== null && !pred(rest.head)) {
    rest = streamCdr(rest);
  }
  if (rest === null) {
    return null;
  }
  const hit = rest;
  return consStream(hit.head, () => filterTriples(streamCdr(hit), pred));
};

/** The statement's Pythagorean triples: the triples of the integers
 * whose i^2 + j^2 = k^2. */
export const pythagoreanTriples: Stream<Triple> = filterTriples(
  triples(integers, integers, integers),
  ([i, j, k]) => i * i + j * j === k * k,
);
