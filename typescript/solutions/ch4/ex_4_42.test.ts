// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_42, solutions } from "./ex_4_42.js";

describe("exercise 4.42: the Liars puzzle", () => {
  it("answers the unique order", async () => {
    expect(await Effect.runPromise(solutions())).toStrictEqual([
      "((betty 3) (ethel 5) (joan 2) (kitty 1) (mary 4))",
    ]);
  });

  it("reports the answer", () => {
    expect(ex_4_42()).toContain("Kitty first");
  });
});
