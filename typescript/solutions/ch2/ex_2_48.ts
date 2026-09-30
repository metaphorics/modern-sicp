// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Vect2 } from "./ex_2_46.js";

/**
 * Exercise 2.48: a representation of segments in terms of exercise
 * 2.46's vectors, with `makeSegment` and the selectors
 * `start-segment` and `end-segment`. A segment is the pair of its
 * endpoint vectors; together with exercise 2.47's frames, the vector
 * -> segment -> frame layering is complete.
 */
export type Segment2 = readonly [Vect2, Vect2];

/** The book's `makeSegment` over the pair representation. */
export const makeSegment2 = (start: Vect2, end: Vect2): Segment2 => [start, end];

/** The book's `start-segment`. */
export const startSegment2 = (segment: Segment2): Vect2 => segment[0];

/** The book's `end-segment`. */
export const endSegment2 = (segment: Segment2): Vect2 => segment[1];
