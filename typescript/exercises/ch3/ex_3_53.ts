// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.53: the self-referential doubling stream. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_53.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.53 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The statement's stream: 1 with the stream added to itself. */
export function s(): Stream<number> {
  throw new PendingSolution();
}

/** The same stream read as a doubling: 1 followed by double scaled
 * by 2. */
export function double(): Stream<number> {
  throw new PendingSolution();
}
