// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeMonitored } from "./ex_3_02.js";

describe("exercise 3.2: make-monitored", () => {
  it.effect("counts calls and answers the book's sqrt example", () =>
    Effect.gen(function* () {
      const s = makeMonitored(Math.sqrt);
      expect(yield* s({ _tag: "Call", arg: 100 })).toBe(10);
      expect(yield* s({ _tag: "HowManyCalls" })).toBe(1);
      expect(yield* s({ _tag: "Call", arg: 16 })).toBe(4);
      expect(yield* s({ _tag: "HowManyCalls" })).toBe(2);
    }),
  );

  it.effect("reset-count zeroes the counter without touching f", () =>
    Effect.gen(function* () {
      const s = makeMonitored(Math.sqrt);
      yield* s({ _tag: "Call", arg: 100 });
      expect(yield* s({ _tag: "ResetCount" })).toBe(0);
      expect(yield* s({ _tag: "HowManyCalls" })).toBe(0);
      expect(yield* s({ _tag: "Call", arg: 9 })).toBe(3);
      expect(yield* s({ _tag: "HowManyCalls" })).toBe(1);
    }),
  );

  it.effect("bookkeeping requests do not invoke f", () =>
    Effect.gen(function* () {
      let calls = 0;
      const counted = (arg: number): number => {
        calls += 1;
        return arg * arg;
      };
      const m = makeMonitored(counted);
      expect(yield* m({ _tag: "Call", arg: 5 })).toBe(25);
      yield* m({ _tag: "HowManyCalls" });
      yield* m({ _tag: "ResetCount" });
      yield* m({ _tag: "HowManyCalls" });
      expect(calls).toBe(1);
      expect(yield* m({ _tag: "HowManyCalls" })).toBe(0);
    }),
  );
});
