// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect, Ref } from "effect";
import { describe, expect } from "vitest";

import { ex_4_23, runWithAlyssaSequence, runWithTextSequence } from "./ex_4_23.js";

const runs = 5;

describe("exercise 4.23: analyze-sequence comparison", () => {
  it.effect("one-expression body: text folds to the leaf, Alyssa walks once per run", () =>
    Effect.gen(function* () {
      const text = yield* runWithTextSequence("(begin 1)", runs);
      expect(text.value).toStrictEqual({ _tag: "Number", n: 1 });
      expect(yield* Ref.get(text.counters.leafRuns)).toBe(runs);
      expect(yield* Ref.get(text.counters.walks)).toBe(0);

      const alyssa = yield* runWithAlyssaSequence("(begin 1)", runs);
      expect(alyssa.value).toStrictEqual({ _tag: "Number", n: 1 });
      expect(yield* Ref.get(alyssa.counters.leafRuns)).toBe(runs);
      expect(yield* Ref.get(alyssa.counters.walks)).toBe(runs);
    }),
  );

  it.effect("two-expression body: identical leaf runs, Alyssa still walks per run", () =>
    Effect.gen(function* () {
      const text = yield* runWithTextSequence("(begin 1 2)", runs);
      expect(text.value).toStrictEqual({ _tag: "Number", n: 2 });
      expect(yield* Ref.get(text.counters.leafRuns)).toBe(2 * runs);
      expect(yield* Ref.get(text.counters.walks)).toBe(0);

      const alyssa = yield* runWithAlyssaSequence("(begin 1 2)", runs);
      expect(alyssa.value).toStrictEqual({ _tag: "Number", n: 2 });
      expect(yield* Ref.get(alyssa.counters.leafRuns)).toBe(2 * runs);
      expect(yield* Ref.get(alyssa.counters.walks)).toBe(runs);
    }),
  );

  it("answers the statement's comparison", () => {
    expect(ex_4_23()).toContain("no sequence work at all");
  });
});
