// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_68, reverseAnswers } from "./ex_4_68.js";

describe("exercise 4.68: reverse as rules", () => {
  it("reverses forward to exactly one answer", () => {
    expect(reverseAnswers("(reverse (1 2 3) ?x)")).toStrictEqual(["(reverse (1 2 3) (3 2 1))"]);
  });

  it("derives the genuine answer backward before the stream runs on", () => {
    expect(reverseAnswers("(reverse ?x (1 2 3))", 1)).toStrictEqual(["(reverse (3 2 1) (1 2 3))"]);
    expect(ex_4_68()).toContain("(reverse (1 2 3) (3 2 1))");
  });
});
