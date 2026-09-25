// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect, Semaphore as EffectSemaphore } from "effect";
import { describe, expect } from "vitest";

import {
  makeSemaphoreFromMutexes,
  makeSemaphoreFromTestAndSet,
  maxConcurrentHolders,
  type Semaphore,
} from "./ex_3_47.js";

describe("exercise 3.47: semaphores from mutexes and test-and-set", () => {
  it.effect("the mutex-built semaphore of size 2 caps four workers at two holders", () =>
    Effect.gen(function* () {
      const semaphore = makeSemaphoreFromMutexes(2);
      expect(yield* maxConcurrentHolders(semaphore, 4)).toBe(2);
    }),
  );

  it.effect("the test-and-set-built semaphore of size 3 caps five workers at three", () =>
    Effect.gen(function* () {
      const semaphore = makeSemaphoreFromTestAndSet(3);
      expect(yield* maxConcurrentHolders(semaphore, 5)).toBe(3);
    }),
  );

  it.effect("tryAcquire answers at once: true while permits remain, false when out", () =>
    Effect.gen(function* () {
      const semaphore = makeSemaphoreFromMutexes(2);
      expect(yield* semaphore.tryAcquire).toBe(true);
      expect(yield* semaphore.tryAcquire).toBe(true);
      expect(yield* semaphore.tryAcquire).toBe(false);
      yield* semaphore.release;
      expect(yield* semaphore.tryAcquire).toBe(true);
    }),
  );

  it.effect("every waiter completes: five acquirers on a size-2 semaphore all finish", () =>
    Effect.gen(function* () {
      const semaphore = makeSemaphoreFromTestAndSet(2);
      let completed = 0;
      const worker = Effect.gen(function* () {
        yield* semaphore.acquire;
        yield* Effect.yieldNow;
        completed += 1;
        yield* semaphore.release;
      });
      const fibers: Array<Effect.Effect<void>> = [];
      for (let index = 0; index < 5; index++) {
        fibers.push(worker);
      }
      yield* Effect.all(fibers, { concurrency: "unbounded", discard: true });
      expect(completed).toBe(5);
    }),
  );

  it.effect("Effect's own Semaphore holds the same bound the builds answer to", () =>
    Effect.gen(function* () {
      // The same probe against Effect's library semaphore, adapted to
      // the exercise's interface: the builds behave like the
      // primitive they reimplement.
      const semaphore = EffectSemaphore.makeUnsafe(2);
      const theirs: Semaphore = {
        acquire: Effect.asVoid(EffectSemaphore.take(semaphore, 1)),
        release: Effect.asVoid(EffectSemaphore.release(semaphore, 1)),
        tryAcquire: EffectSemaphore.takeIfAvailable(semaphore, 1),
      };
      expect(yield* maxConcurrentHolders(theirs, 4)).toBe(2);
    }),
  );
});
