// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_52, run } from "./ex_4_52.js";

describe("exercise 4.52: if-fail", () => {
  it("no even element: all-odd, then exhaustion", async () => {
    expect(await Effect.runPromise(run("1 3 5"))).toStrictEqual(["all-odd"]);
  });

  it("with 8: the value 8, then the fallback all-odd, then exhaustion", async () => {
    expect(await Effect.runPromise(run("1 3 5 8"))).toStrictEqual(["8", "all-odd"]);
  });

  it("reports the catch", () => {
    expect(ex_4_52()).toContain("stands down");
  });
});
