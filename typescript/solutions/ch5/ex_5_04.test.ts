// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_04 } from "./ex_5_04.ts";

describe("exercise 5.4 exponentiation machines", () => {
  it("recursive and iterative controllers compute the same power", () => {
    const result = ex_5_04(3, 4);
    expect(result.recursive).toBe(81);
    expect(result.iterative).toBe(81);
  });
  it("the exponent-zero boundary answers one on both machines", () => {
    const result = ex_5_04(7, 0);
    expect(result.recursive).toBe(1);
    expect(result.iterative).toBe(1);
  });
});
