// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

/**
 * Exercise 3.47: semaphores from mutexes and test-and-set. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_47.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.47 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The section's semaphore: acquire, release, and a fail-fast
 * tryAcquire. */
export interface Semaphore {
  readonly acquire: Effect.Effect<void>;
  readonly release: Effect.Effect<void>;
  readonly tryAcquire: Effect.Effect<boolean>;
}

/** (a) In terms of mutexes: the permit count and the waiter list live
 * under a mutex; release hands its permit directly to the first
 * waiter, so no woken process can lose the wakeup. */
export function makeSemaphoreFromMutexes(_size: number): Semaphore {
  throw new PendingSolution();
}

/** (b) In terms of atomic test-and-set!: the guard the permit state
 * lives under is spelled from the book's cell and the module's atomic
 * `testAndSet`, with the busy-wait replaced by a rescheduling yield. */
export function makeSemaphoreFromTestAndSet(_size: number): Semaphore {
  throw new PendingSolution();
}

/** Measures the highest number of holders a semaphore's critical
 * sections reach, over `workers` forked acquirers that each spend one
 * suspension inside. */
export function maxConcurrentHolders(
  _semaphore: Semaphore,
  _workers: number,
): Effect.Effect<number> {
  throw new PendingSolution();
}
