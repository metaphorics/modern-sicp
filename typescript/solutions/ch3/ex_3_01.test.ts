// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeAccumulator } from "./ex_3_01.js";

describe("exercise 3.1: make-accumulator", () => {
  it.effect("accumulates the book's 5, 10, 10 example", () =>
    Effect.gen(function* () {
      const a = makeAccumulator(5);
      expect(yield* a(10)).toBe(15);
      expect(yield* a(10)).toBe(25);
    }),
  );

  it.effect("each generated accumulator keeps an independent sum", () =>
    Effect.gen(function* () {
      const a = makeAccumulator(5);
      const b = makeAccumulator(100);
      expect(yield* a(10)).toBe(15);
      expect(yield* b(1)).toBe(101);
      expect(yield* a(5)).toBe(20);
      expect(yield* b(1)).toBe(102);
    }),
  );
});
