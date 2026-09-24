// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.2

import { it } from "@effect/vitest";
import { Effect, Option, Ref } from "effect";
import { expect } from "vitest";

import type { Value } from "./core.js";
import { makeThunk } from "./core.js";
import { UnknownSyntax } from "./errors.js";

it.effect("force runs the thunk once and memoizes the value in its cell", () =>
  Effect.gen(function* () {
    const runs: Ref.Ref<number> = yield* Ref.make(0);
    const answer: Value = { _tag: "Number", n: 41 };
    const thunk = yield* makeThunk(() =>
      Effect.gen(function* () {
        yield* Ref.update(runs, (n) => n + 1);
        return answer;
      }),
    );

    expect(yield* Ref.get(thunk.cell)).toStrictEqual(Option.none());
    const first = yield* thunk.force();
    const second = yield* thunk.force();

    expect(second).toStrictEqual(first);
    expect(yield* Ref.get(thunk.cell)).toStrictEqual(Option.some(answer));
    expect(yield* Ref.get(runs)).toBe(1);
  }),
);

it.effect("a failed force is not memoized, so a later force retries", () =>
  Effect.gen(function* () {
    const runs: Ref.Ref<number> = yield* Ref.make(0);
    const thunk = yield* makeThunk(() =>
      Effect.gen(function* () {
        const count = yield* Ref.updateAndGet(runs, (n) => n + 1);
        if (count === 1) {
          return yield* Effect.fail(new UnknownSyntax({ expr: "boom" }));
        }
        const recovered: Value = { _tag: "Boolean", b: true };
        return recovered;
      }),
    );

    const first = yield* Effect.result(thunk.force());
    const second = yield* Effect.result(thunk.force());

    expect(first._tag).toBe("Failure");
    expect(second._tag).toBe("Success");
    if (second._tag === "Success") {
      expect(second.success).toStrictEqual({ _tag: "Boolean", b: true });
    }
    expect(yield* Ref.get(runs)).toBe(2);
  }),
);
