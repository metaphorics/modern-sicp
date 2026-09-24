// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { Effect, Option } from "effect";
import { describe, expect, it } from "vitest";

import type { Primitive, Value } from "./core.js";
import { OpTable } from "./primitives.js";

const add: Primitive = (args) =>
  Effect.sync(() => {
    let total = 0;
    for (const arg of args) {
      if (arg._tag === "Number") {
        total += Number(arg.n);
      }
    }
    return { _tag: "Number", n: total } satisfies Value;
  });

describe("OpTable", () => {
  it("get on a missing key returns the absent option, never false", () => {
    const table = new OpTable();
    expect(table.get("+")).toStrictEqual(Option.none());
  });

  it("put installs a handler that get hands back", () => {
    const table = new OpTable();
    table.put("+", add);
    const installed = table.get("+");
    expect(Option.isSome(installed)).toBe(true);
    if (Option.isSome(installed)) {
      expect(
        Effect.runSync(
          installed.value([
            { _tag: "Number", n: 2 },
            { _tag: "Number", n: 3 },
          ]),
        ),
      ).toStrictEqual({
        _tag: "Number",
        n: 5,
      });
    }
  });

  it("put overwrites an existing entry (D19)", () => {
    const replacement: Primitive = () =>
      Effect.succeed({ _tag: "Boolean", b: false } satisfies Value);
    const table = new OpTable();
    table.put("+", add);
    table.put("+", replacement);
    const installed = table.get("+");
    if (Option.isSome(installed)) {
      expect(Effect.runSync(installed.value([]))).toStrictEqual({ _tag: "Boolean", b: false });
    } else {
      expect.unreachable("put must install the handler");
    }
  });
});
