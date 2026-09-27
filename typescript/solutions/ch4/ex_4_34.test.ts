// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { answers, ex_4_34 } from "./ex_4_34.js";

describe("exercise 4.34: printing lazy pairs", () => {
  it("prints finite pairs whole and infinite lists under the budget", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed[0]).toBe("(1 2)");
    expect(observed[1]).toBe("ok");
    expect(observed[2]).toBe("(1 1 1 1 1 1 1 1 1 1 ...)");
    expect(observed[3]).toBe("1");
    expect(observed[4]).toBe("((1) 2)");
  });

  it("reports the tagged representation and the budget", () => {
    const report = ex_4_34();
    expect(report).toContain("lazy-pair");
    expect(report).toContain("ten forced elements");
  });
});
