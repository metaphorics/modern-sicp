// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeRand } from "../../packages/ch3/src/01-assignment.js";

import { estimateIntegral, estimatePiByIntegration, randomInRange } from "./ex_3_05.js";

describe("exercise 3.5: Monte Carlo integration", () => {
  it.effect("random-in-range stays inside [low, high) and hits both ends", () =>
    Effect.gen(function* () {
      const rand = makeRand(42);
      const low = yield* randomInRange(2, 8, rand);
      expect(low).toBeCloseTo(2 + 6 * (11355432 / 4294967296), 12);
      for (let i = 0; i < 100; i++) {
        const value = yield* randomInRange(-3, 5, rand);
        expect(value).toBeGreaterThanOrEqual(-3);
        expect(value).toBeLessThan(5);
      }
    }),
  );

  it.effect("a region that is the whole rectangle estimates its exact area", () =>
    Effect.gen(function* () {
      const estimate = yield* estimateIntegral(() => true, 0, 3, 0, 4, 100, makeRand(42));
      expect(estimate).toBe(12);
    }),
  );

  it.effect("an empty region estimates zero", () =>
    Effect.gen(function* () {
      const estimate = yield* estimateIntegral(() => false, 0, 3, 0, 4, 100, makeRand(42));
      expect(estimate).toBe(0);
    }),
  );

  it.effect("the unit circle estimates pi", () =>
    Effect.gen(function* () {
      const estimate = yield* estimatePiByIntegration(10000, 42);
      expect(estimate).toBeCloseTo(3.104, 3);
      expect(Math.abs(estimate - Math.PI)).toBeLessThan(0.1);
    }),
  );

  it.effect("the book's radius-3 circle at (5, 7) estimates 9 pi", () =>
    Effect.gen(function* () {
      const estimate = yield* estimateIntegral(
        (x, y) => (x - 5) * (x - 5) + (y - 7) * (y - 7) <= 9,
        2,
        8,
        4,
        10,
        10000,
        makeRand(42),
      );
      expect(estimate).toBeCloseTo(27.936, 3);
      expect(Math.abs(estimate - 9 * Math.PI)).toBeLessThan(0.5);
    }),
  );

  it.effect("the same seed reproduces the same estimate", () =>
    Effect.gen(function* () {
      const first = yield* estimatePiByIntegration(5000, 7);
      const second = yield* estimatePiByIntegration(5000, 7);
      expect(first).toBe(second);
    }),
  );
});
