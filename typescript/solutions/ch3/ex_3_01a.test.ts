// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeAccumulatorWithHistory } from "./ex_3_01a.js";

describe("exercise 3.1a: accumulator with transaction history", () => {
  it.effect("answers the history that produced each sum", () =>
    Effect.gen(function* () {
      const a = makeAccumulatorWithHistory(5);
      expect(yield* a(10)).toEqual({ sum: 15, transactions: [10] });
      expect(yield* a(10)).toEqual({ sum: 25, transactions: [10, 10] });
    }),
  );

  it.effect("keeps the initial value out of the transaction history", () =>
    Effect.gen(function* () {
      const a = makeAccumulatorWithHistory(100);
      expect(yield* a(1)).toEqual({ sum: 101, transactions: [1] });
      expect(yield* a(2)).toEqual({ sum: 103, transactions: [1, 2] });
    }),
  );

  it.effect("two accumulators keep independent histories", () =>
    Effect.gen(function* () {
      const a = makeAccumulatorWithHistory(0);
      const b = makeAccumulatorWithHistory(0);
      yield* a(3);
      expect(yield* b(4)).toEqual({ sum: 4, transactions: [4] });
      expect(yield* a(3)).toEqual({ sum: 6, transactions: [3, 3] });
    }),
  );
});
