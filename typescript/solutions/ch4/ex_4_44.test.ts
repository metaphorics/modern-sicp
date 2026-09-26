// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { bruteForceCount, ex_4_44, solutions } from "./ex_4_44.js";

describe("exercise 4.44: queens under amb", () => {
  it("the 8x8 board answers 92 solutions with the pinned first board", {
    timeout: 60000,
  }, async () => {
    const all = await Effect.runPromise(solutions(8));
    expect(all).toHaveLength(92);
    expect(all[0]).toBe("(4 2 7 3 6 8 5 1)");
    expect(bruteForceCount(8)).toBe(92);
  });

  it("the small boards answer the classic first solutions", { timeout: 60000 }, async () => {
    expect(await Effect.runPromise(solutions(4))).toStrictEqual(["(3 1 4 2)", "(2 4 1 3)"]);
    expect(await Effect.runPromise(solutions(6))).toStrictEqual([
      "(5 3 1 6 4 2)",
      "(4 1 5 2 6 3)",
      "(3 6 2 5 1 4)",
      "(2 4 6 1 3 5)",
    ]);
  });

  it("reports the count", { timeout: 120000 }, () => {
    expect(ex_4_44()).toContain("92 solutions");
  });
});
