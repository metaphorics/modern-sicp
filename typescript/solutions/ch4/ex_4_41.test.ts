// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_41, render, solutions } from "./ex_4_41.js";

describe("exercise 4.41: an ordinary program solves the puzzle", () => {
  it("answers exactly the amb evaluator's unique solution", () => {
    expect(solutions().map(render)).toStrictEqual([
      "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))",
    ]);
  });

  it("enumerates 120 permutations", () => {
    expect(solutions()).toHaveLength(1);
  });

  it("reports the answer", () => {
    expect(ex_4_41()).toContain("notation, not power");
  });
});
