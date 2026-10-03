// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { answers } from "./ex_4_34.js";

describe("exercise 4.34: printing lazy pairs", () => {
  it("prints the finite pair, the infinite ones, a demand, and a nested pair", () => {
    const observed = answers();
    expect(observed.transcript).toEqual([
      "[1, 2]",
      "ok",
      "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ...]",
      "1",
      "[[1], 2]",
    ]);
    expect(observed.outcome.tag).toBe("ok");
  });
});
