// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list } from "../../packages/ch2/src/02-picture-language.js";
import { hornerEval } from "./ex_2_34.js";

describe("exercise 2.34", () => {
  it("evaluates 1 + 3x + 5x^3 + x^5 at x = 2 by Horner's rule", () => {
    expect(hornerEval(2, list(1, 3, 0, 5, 0, 1))).toBe(79);
  });
});
