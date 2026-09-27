// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { memoizedRun, unmemoizedRun } from "./ex_3_52.js";

describe("exercise 3.52: accum traces assignment plus laziness", () => {
  it.live("the memoized delay answers the book's transcript", () =>
    Effect.gen(function* () {
      const run = yield* memoizedRun();
      expect(run.sumAfterSeq).toBe(1);
      expect(run.sumAfterY).toBe(6);
      expect(run.sumAfterZ).toBe(10);
      expect(run.refAnswer).toBe(136);
      expect(run.sumAfterRef).toBe(136);
      expect(run.zDisplayed).toEqual([10, 15, 45, 55, 105, 120, 190, 210]);
      expect(run.sumAfterDisplay).toBe(210);
    }),
  );

  it.live("the plain-thunk delay answers differently, so memoization matters", () =>
    Effect.gen(function* () {
      const memoized = yield* memoizedRun();
      const plain = yield* unmemoizedRun();
      expect(plain.sumAfterSeq).toBe(1);
      expect(plain.sumAfterY).toBe(6);
      expect(plain.sumAfterZ).toBe(15);
      expect(plain.refAnswer).toBe(162);
      expect(plain.sumAfterRef).toBe(162);
      expect(plain.zDisplayed).toEqual([15, 180, 230, 305]);
      expect(plain.sumAfterDisplay).toBe(362);
      expect(plain.sumAfterRef).not.toBe(memoized.sumAfterRef);
      expect(plain.sumAfterDisplay).not.toBe(memoized.sumAfterDisplay);
    }),
  );
});
