// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Vect2 } from "./ex_2_46.js";

/** Exercise 2.48: segments over exercise 2.46's vectors. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.48 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A segment: the pair (start end) of endpoint vectors. */
export type Segment2 = readonly [Vect2, Vect2];

/** The book's make-segment over the pair representation. */
export function makeSegment2(_start: Vect2, _end: Vect2): Segment2 {
  throw new PendingSolution();
}

/** The book's start-segment. */
export function startSegment2(_segment: Segment2): Vect2 {
  throw new PendingSolution();
}

/** The book's end-segment. */
export function endSegment2(_segment: Segment2): Vect2 {
  throw new PendingSolution();
}
