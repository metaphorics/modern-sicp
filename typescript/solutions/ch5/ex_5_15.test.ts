// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  ex_5_15,
  factorialInstructionCount,
  fibInstructionCount,
  gcdInstructionCounts,
  printAndReset,
} from "./ex_5_15.ts";

describe("exercise 5.15 instruction counting", () => {
  it("counts every executed instruction, transfers included", () => {
    expect(gcdInstructionCounts(206, 40)).toBe(26);
    expect(factorialInstructionCount(5)).toBe(49);
  });
  it("the Fibonacci boundaries of the book's table", () => {
    expect(fibInstructionCount(0)).toBe(5);
    expect(fibInstructionCount(1)).toBe(5);
    expect(fibInstructionCount(2)).toBe(28);
    expect(fibInstructionCount(3)).toBe(51);
    expect(fibInstructionCount(6)).toBe(281);
  });
  it("print-and-reset measures only later runs", () => {
    const result = printAndReset();
    expect(result.message).toBe(27);
    expect(result.transcript).toEqual(["27"]);
    expect(result.after).toBe(0);
  });
  it("the answer table is complete", () => {
    expect(ex_5_15()).toHaveLength(7);
  });
});
