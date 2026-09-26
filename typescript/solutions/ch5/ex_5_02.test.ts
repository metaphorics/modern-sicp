// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_02 } from "./ex_5_02.js";

describe("exercise 5.2 iterative factorial controller", () => {
  it("answers one for zero and factorial for positive input", () => {
    const zero = ex_5_02(0);
    const six = ex_5_02(6);
    expect(zero.ok && zero.value.registers["product"]).toBe(1);
    expect(six.ok && six.value.registers["product"]).toBe(720);
  });
});
