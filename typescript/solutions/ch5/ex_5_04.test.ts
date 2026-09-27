// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_04 } from "./ex_5_04.js";

describe("exercise 5.4 exponentiation machines", () => {
  it("recursive and iterative controllers compute the same power", () => {
    const result = ex_5_04(3, 4);
    expect(result.recursive.ok && result.recursive.value.registers["val"]).toBe(81);
    expect(result.iterative.ok && result.iterative.value.registers["product"]).toBe(81);
  });
});
