// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Deferred, Effect } from "effect";

import { makeCell, makeMutex, testAndSet } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.47: a semaphore of size n from (a) mutexes and (b) atomic
 * `test-and-set!`. Both are blocking bounded semaphores: acquire
 * parks the fiber when no permit is free, release hands the freed
 * permit straight to a waiter, and tryAcquire answers immediately.
 * The map's tailored note is honored by construction: Effect ships a
 * `Semaphore`, and the exercise builds the mechanism anyway, then the
 * tests hold both editions' semaphores to the same bound.
 */

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
export const makeSemaphoreFromMutexes = (size: number): Semaphore => {
  const mutex = makeMutex();
  const state = { permits: size, waiters: [] as Deferred.Deferred<void>[] };
  const acquire: Effect.Effect<void> = Effect.gen(function* () {
    yield* mutex.acquire;
    if (state.permits > 0) {
      state.permits -= 1;
      yield* mutex.release;
      return;
    }
    const waiter = yield* Deferred.make<void>();
    state.waiters.push(waiter);
    yield* mutex.release;
    yield* Deferred.await(waiter);
  });
  const release: Effect.Effect<void> = Effect.gen(function* () {
    yield* mutex.acquire;
    const next = state.waiters.shift();
    if (next !== undefined) {
      yield* mutex.release;
      yield* Deferred.succeed(next, undefined);
      return;
    }
    state.permits += 1;
    yield* mutex.release;
  });
  const tryAcquire: Effect.Effect<boolean> = Effect.gen(function* () {
    yield* mutex.acquire;
    if (state.permits > 0) {
      state.permits -= 1;
      yield* mutex.release;
      return true;
    }
    yield* mutex.release;
    return false;
  });
  return { acquire, release, tryAcquire };
};

/** (b) In terms of atomic test-and-set!: the guard the permit state
 * lives under is spelled from the book's cell and the module's atomic
 * `testAndSet`, the same construction the text's `make-mutex` makes,
 * with the busy-wait replaced by a rescheduling yield. */
export const makeSemaphoreFromTestAndSet = (size: number): Semaphore => {
  const cell = makeCell(false);
  const state = { permits: size, waiters: [] as Deferred.Deferred<void>[] };
  const enterGuard = Effect.gen(function* () {
    for (;;) {
      if (!testAndSet(cell)) {
        return;
      }
      yield* Effect.yieldNow;
    }
  });
  const exitGuard = Effect.sync(() => {
    cell.contents = false;
  });
  const acquire: Effect.Effect<void> = Effect.gen(function* () {
    yield* enterGuard;
    if (state.permits > 0) {
      state.permits -= 1;
      yield* exitGuard;
      return;
    }
    const waiter = yield* Deferred.make<void>();
    state.waiters.push(waiter);
    yield* exitGuard;
    yield* Deferred.await(waiter);
    // The release that woke us handed its permit straight over, so the
    // permit count is still ours to have.
  });
  const release: Effect.Effect<void> = Effect.gen(function* () {
    yield* enterGuard;
    const next = state.waiters.shift();
    if (next !== undefined) {
      yield* exitGuard;
      yield* Deferred.succeed(next, undefined);
      return;
    }
    state.permits += 1;
    yield* exitGuard;
  });
  const tryAcquire: Effect.Effect<boolean> = Effect.gen(function* () {
    yield* enterGuard;
    if (state.permits > 0) {
      state.permits -= 1;
      yield* exitGuard;
      return true;
    }
    yield* exitGuard;
    return false;
  });
  return { acquire, release, tryAcquire };
};

/** Measures the highest number of holders a semaphore's critical
 * sections reach, over `workers` forked acquirers that each spend one
 * suspension inside. */
export const maxConcurrentHolders = (
  semaphore: Semaphore,
  workers: number,
): Effect.Effect<number> =>
  Effect.gen(function* () {
    const inside = { count: 0, max: 0 };
    const worker = Effect.gen(function* () {
      yield* semaphore.acquire;
      inside.count += 1;
      if (inside.count > inside.max) {
        inside.max = inside.count;
      }
      yield* Effect.yieldNow;
      inside.count -= 1;
      yield* semaphore.release;
    });
    const fibers: Array<Effect.Effect<void>> = [];
    for (let index = 0; index < workers; index++) {
      fibers.push(worker);
    }
    yield* parallelJoin(fibers);
    return inside.max;
  });

const parallelJoin = (effects: ReadonlyArray<Effect.Effect<void>>): Effect.Effect<void> =>
  Effect.all(effects, { concurrency: "unbounded", discard: true });
