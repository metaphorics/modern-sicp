// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { Cell, Mutex } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.46: the test-and-set race window. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_46.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.46 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `test-and-set!` as the text's ordinary procedure, with
 * the window between the test and the set made an explicit
 * suspension: read the cell, suspend, then write it. */
export function racyTestAndSet(_cell: Cell): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** The book's `make-mutex` over the racy test-and-set: a fiber that
 * finds the cell true suspends and retries until a release clears the
 * cell, but its acquire can now report success to two fibers at
 * once. */
export function makeRacyMutex(): Mutex {
  throw new PendingSolution();
}

/** The stress: `trials` two-process acquisitions of the racy mutex,
 * answering how many trials saw both processes inside the critical
 * section together. */
export function doubleAcquireTrials(_trials: number): Effect.Effect<number> {
  throw new PendingSolution();
}

/** The control with the module's atomic mutex: the same stress, and no
 * trial ever sees both processes inside. */
export function atomicDoubleAcquireTrials(_trials: number): Effect.Effect<number> {
  throw new PendingSolution();
}
