// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import type { Value } from "./core.js";
import { defineVariable, extendEnv, lookupVariable, makeGlobalEnv, setVariable } from "./env.js";
import { UnboundVariable } from "./errors.js";

const number: Value = { _tag: "Number", n: 16 };
const symbol: Value = { _tag: "Symbol", name: "square" };

describe("Env", () => {
  it.effect("defines in the global frame and looks the name back up", () =>
    Effect.gen(function* () {
      const env = yield* makeGlobalEnv();
      yield* defineVariable(env, "x", number);
      expect(yield* lookupVariable(env, "x")).toStrictEqual(number);
    }),
  );

  it.effect("walks the chain: a child frame sees its parent's names", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalEnv();
      yield* defineVariable(global, "x", number);
      const child = yield* extendEnv(global);
      expect(yield* lookupVariable(child, "x")).toStrictEqual(number);
    }),
  );

  it.effect("a child define shadows the parent frame without touching it", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalEnv();
      yield* defineVariable(global, "x", number);
      const shadowed: Value = { _tag: "Number", n: 25 };
      const child = yield* extendEnv(global);
      yield* defineVariable(child, "x", shadowed);
      expect(yield* lookupVariable(child, "x")).toStrictEqual(shadowed);
      expect(yield* lookupVariable(global, "x")).toStrictEqual(number);
    }),
  );

  it.effect("set! mutates the frame that defines the name, and the change escapes the call", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalEnv();
      yield* defineVariable(global, "balance", number);
      const callFrame = yield* extendEnv(global);

      yield* setVariable(callFrame, "balance", symbol);
      expect(yield* lookupVariable(global, "balance")).toStrictEqual(symbol);
      expect(yield* lookupVariable(callFrame, "balance")).toStrictEqual(symbol);
    }),
  );

  it.effect("lookup of an unknown name fails with UnboundVariable carrying the name", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalEnv();
      const missed = yield* Effect.result(lookupVariable(global, "nope"));
      expect(missed._tag).toBe("Failure");
      if (missed._tag === "Failure") {
        expect(missed.failure._tag).toBe("UnboundVariable");
        if (missed.failure._tag === "UnboundVariable") {
          expect(missed.failure.name).toBe("nope");
        }
      }
    }),
  );

  it.effect("set! of an unbound name fails instead of defining it", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalEnv();
      const unset = yield* Effect.result(setVariable(global, "nope", number));
      expect(unset._tag).toBe("Failure");
      if (unset._tag === "Failure") {
        expect(unset.failure).toBeInstanceOf(UnboundVariable);
      }
    }),
  );

  it.effect(
    "two calls from one base frame stay independent (3.11a: two accounts share no state)",
    () =>
      Effect.gen(function* () {
        const base = yield* makeGlobalEnv();
        const accountA = yield* extendEnv(base);
        const accountB = yield* extendEnv(base);
        yield* defineVariable(accountA, "balance", number);
        yield* defineVariable(accountB, "balance", symbol);
        expect(yield* lookupVariable(accountA, "balance")).toStrictEqual(number);
        expect(yield* lookupVariable(accountB, "balance")).toStrictEqual(symbol);
      }),
  );
});
