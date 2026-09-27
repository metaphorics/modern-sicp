// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { answers, ex_4_51 } from "./ex_4_51.js";

describe("exercise 4.51: permanent-set!", () => {
  it("permanent-set! counts every trial; set! undoes on backtrack", async () => {
    expect(await Effect.runPromise(answers("permanent-set!"))).toStrictEqual([
      "(a b 2)",
      "(a c 3)",
      "(b a 4)",
      "(b c 6)",
      "(c a 7)",
      "(c b 8)",
    ]);
    expect(await Effect.runPromise(answers("set!"))).toStrictEqual([
      "(a b 1)",
      "(a c 1)",
      "(b a 1)",
      "(b c 1)",
      "(c a 1)",
      "(c b 1)",
    ]);
  });

  it("reports the interaction", () => {
    expect(ex_4_51()).toContain("undo record");
  });
});
