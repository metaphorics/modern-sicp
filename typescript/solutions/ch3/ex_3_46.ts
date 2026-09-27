// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber } from "effect";

import {
  type Cell,
  type Mutex,
  makeCell,
  makeMutex,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.46: the test-and-set race window. The book's plain
 * `test-and-set!` reads the cell and later writes it; between those
 * two steps lies the window the timing diagrams draw. The edition's
 * single-threaded runtime never preempts a synchronous read and write,
 * so the racy variant here makes the window an explicit fiber
 * suspension. A stress of trials then counts how often two processes
 * both acquire the mutex built on it, against the atomic control.
 */

/** The book's `test-and-set!` as the text's ordinary procedure, with
 * the window between the test and the set made an explicit
 * suspension: read the cell, suspend, then write it. */
export const racyTestAndSet = (cell: Cell): Effect.Effect<boolean> =>
  Effect.gen(function* () {
    const wasTrue = cell.contents;
    yield* Effect.yieldNow;
    if (!wasTrue) {
      cell.contents = true;
    }
    return wasTrue;
  });

/** The book's `make-mutex` over the racy test-and-set: a fiber that
 * finds the cell true yields (the parked retry, not a burning loop)
 * until a release clears the cell, but its acquire can now report
 * success to two fibers at once. */
export const makeRacyMutex = (): Mutex => {
  const cell = makeCell(false);
  return {
    acquire: Effect.gen(function* () {
      for (;;) {
        const lost = yield* racyTestAndSet(cell);
        if (!lost) {
          return;
        }
        yield* Effect.yieldNow;
      }
    }),
    release: Effect.sync(() => {
      cell.contents = false;
    }),
  };
};

/** One trial with the given mutex: two processes each acquire, mark
 * themselves inside, and release; the trial fails when both are inside
 * at once. */
const bothInsideOnce = (mutex: Mutex): Effect.Effect<boolean> =>
  Effect.gen(function* () {
    const inside = { count: 0, both: false };
    const worker = Effect.gen(function* () {
      yield* mutex.acquire;
      inside.count += 1;
      yield* Effect.yieldNow;
      if (inside.count === 2) {
        inside.both = true;
      }
      inside.count -= 1;
      yield* mutex.release;
    });
    const first = yield* Effect.forkChild(worker);
    const second = yield* Effect.forkChild(worker);
    yield* Fiber.join(first);
    yield* Fiber.join(second);
    return inside.both;
  });

/** The stress: `trials` two-process acquisitions of the racy mutex,
 * answering how many trials saw both processes inside the critical
 * section together. Every trial fails: the window swallows both. */
export const doubleAcquireTrials = (trials: number): Effect.Effect<number> =>
  Effect.gen(function* () {
    let failures = 0;
    for (let trial = 0; trial < trials; trial++) {
      const both = yield* bothInsideOnce(makeRacyMutex());
      if (both) {
        failures += 1;
      }
    }
    return failures;
  });

/** The control with the module's atomic mutex: the same stress, and no
 * trial ever sees both processes inside. */
export const atomicDoubleAcquireTrials = (trials: number): Effect.Effect<number> =>
  Effect.gen(function* () {
    let failures = 0;
    for (let trial = 0; trial < trials; trial++) {
      const both = yield* bothInsideOnce(makeMutex());
      if (both) {
        failures += 1;
      }
    }
    return failures;
  });
