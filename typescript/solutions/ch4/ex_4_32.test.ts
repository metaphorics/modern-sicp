// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { answers, ex_4_32 } from "./ex_4_32.js";

describe("exercise 4.32: streams versus lazy lists", () => {
  it("pins both demand disciplines and the eager contrast", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed[0]).toBe("7");
    expect(observed[1]).toBe("/: expects a nonzero divisor list");
    expect(observed[2]).toBe("/: expects a nonzero divisor list");
    expect(observed[3]).toBe("ok");
  });

  it("explains the lazier car slot", () => {
    expect(ex_4_32()).toContain("car slot");
  });
});
