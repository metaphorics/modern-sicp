// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_53, result } from "./ex_4_53.js";

describe("exercise 4.53: permanent-set! under if-fail", () => {
  it("accumulates the three prime-sum pairs, then runs dry", async () => {
    const observed = await Effect.runPromise(result());
    expect(observed.value).toBe("((8 35) (3 110) (3 20))");
    expect(observed.exhausted).toBe(true);
  });

  it("reports the accumulation", () => {
    expect(ex_4_53()).toContain("survives the backtrack");
  });
});
