// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { ex_4_19, runDebatedProgram } from "./ex_4_19.js";

describe("exercise 4.19: the internal definition scoping debate", () => {
  it.effect("the sequential rule gives Ben's 16", () =>
    Effect.gen(function* () {
      expect(yield* runDebatedProgram("sequential")).toStrictEqual({ _tag: "Number", n: 16 });
    }),
  );

  it.effect("Alyssa's scanned rule errors on the unassigned read", () =>
    Effect.gen(function* () {
      const outcome = yield* Effect.result(runDebatedProgram("scanned"));
      expect(outcome._tag).toBe("Failure");
      if (outcome._tag === "Failure") {
        expect(outcome.failure._tag).toBe("RuntimeError");
        if (outcome.failure._tag === "RuntimeError") {
          expect(outcome.failure.message).toBe("variable used before assignment: a");
        }
      }
    }),
  );

  it.effect("Eva's rule computes with final values and gives 20", () =>
    Effect.gen(function* () {
      expect(yield* runDebatedProgram("eva")).toStrictEqual({ _tag: "Number", n: 20 });
    }),
  );

  it("takes a position", () => {
    expect(ex_4_19()).toContain("I support the Alyssa/MIT position");
  });
});
