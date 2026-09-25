// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeResettableRand } from "./ex_3_06.js";

describe("exercise 3.6: generate and reset rand", () => {
  it.effect("resetting to the seed replays the same sequence", () =>
    Effect.gen(function* () {
      const rand = makeResettableRand(42);
      expect(yield* rand({ _tag: "Generate" })).toBe(11355432);
      expect(yield* rand({ _tag: "Generate" })).toBe(2836018348);
      expect(yield* rand({ _tag: "Reset", newValue: 42 })).toBe(42);
      expect(yield* rand({ _tag: "Generate" })).toBe(11355432);
      expect(yield* rand({ _tag: "Generate" })).toBe(2836018348);
    }),
  );

  it.effect("resetting mid-sequence restarts from the new value", () =>
    Effect.gen(function* () {
      const rand = makeResettableRand(42);
      yield* rand({ _tag: "Generate" });
      expect(yield* rand({ _tag: "Reset", newValue: 7 })).toBe(7);
      expect(yield* rand({ _tag: "Generate" })).toBe(1892583);
    }),
  );

  it.effect("two generators keep independent state", () =>
    Effect.gen(function* () {
      const r1 = makeResettableRand(42);
      const r2 = makeResettableRand(7);
      expect(yield* r1({ _tag: "Generate" })).toBe(11355432);
      expect(yield* r2({ _tag: "Generate" })).toBe(1892583);
      yield* r1({ _tag: "Reset", newValue: 7 });
      expect(yield* r1({ _tag: "Generate" })).toBe(1892583);
      expect(yield* r2({ _tag: "Generate" })).toBe(470389255);
    }),
  );
});
