// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { gcdController } from "./ex_5_07.ts";
import { controllerRegisters, ex_5_13 } from "./ex_5_13.ts";

describe("exercise 5.13 the controller decides the registers", () => {
  it("the derived list names exactly the controller's registers", () => {
    expect(controllerRegisters(gcdController)).toEqual(["a", "b", "t"]);
  });
  it("the derived machine answers like the hand-listed one", () => {
    const result = ex_5_13(206, 40);
    expect(result.registers).toEqual(["a", "b", "t"]);
    expect(result.answer).toBe(2);
  });
});
