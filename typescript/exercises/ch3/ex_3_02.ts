// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

/**
 * Exercise 3.2: make-monitored, a wrapper counting the calls it
 * answers and resetting on request. Pending scaffold; the solution and
 * its rationale live in solutions/ch3/ex_3_02.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.2 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Wraps `f` in a procedure that counts and can reset its counter. */
export function makeMonitored<A, B>(
  _f: (arg: A) => B,
): (request: MonitorRequest<A>) => Effect.Effect<B | number> {
  throw new PendingSolution();
}

/** The request union: call f, ask for the count, or reset it. */
export type MonitorRequest<A> =
  | { readonly _tag: "Call"; readonly arg: A }
  | { readonly _tag: "HowManyCalls" }
  | { readonly _tag: "ResetCount" };
