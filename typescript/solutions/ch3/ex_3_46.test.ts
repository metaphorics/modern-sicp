// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";
import { makeCell } from "../../packages/ch3/src/04-concurrency.js";
import { atomicDoubleAcquireTrials, doubleAcquireTrials, racyTestAndSet } from "./ex_3_46.js";

describe("exercise 3.46: test-and-set race window", () => {
  it.effect("the racy test-and-set claims a free cell", () =>
    Effect.gen(function* () {
      const cell = makeCell(false);
      expect(yield* racyTestAndSet(cell)).toBe(false);
      expect(cell.contents).toBe(true);
      expect(yield* racyTestAndSet(cell)).toBe(true);
    }),
  );

  it.effect("every racy-mutex trial lets both processes acquire together", () =>
    Effect.gen(function* () {
      // 25 trials, two processes each: both read false inside the
      // window, both write true, both believe they hold the mutex.
      expect(yield* doubleAcquireTrials(25)).toBe(25);
    }),
  );

  it.effect("the atomic mutex never does, across the same stress", () =>
    Effect.gen(function* () {
      expect(yield* atomicDoubleAcquireTrials(25)).toBe(0);
    }),
  );
});
