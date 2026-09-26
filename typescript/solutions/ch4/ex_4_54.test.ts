// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_54, prunedEvens, requireFalse, requireTrue } from "./ex_4_54.js";

describe("exercise 4.54: require as a special form", () => {
  it("a true require answers ok, a false one exhausts", async () => {
    expect(await Effect.runPromise(requireTrue())).toStrictEqual(["ok"]);
    expect(await Effect.runPromise(requireFalse())).toStrictEqual([]);
  });

  it("inside a choice the special form prunes like the procedure", async () => {
    expect(await Effect.runPromise(prunedEvens())).toStrictEqual(["2", "4"]);
  });

  it("reports the completion", () => {
    expect(ex_4_54()).toContain("isTrue");
  });
});
