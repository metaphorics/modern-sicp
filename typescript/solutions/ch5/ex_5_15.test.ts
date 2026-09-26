// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { setRegisterContents } from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";
import { fibSimController } from "./ex_5_11.js";
import { gcdInstructionCounts, makeCountingMachine, printAndReset } from "./ex_5_15.js";

describe("exercise 5.15 instruction counting", () => {
  it("totals every executed instruction, transfers included", () => {
    expect(gcdInstructionCounts()).toEqual([
      "gcd(206, 40): 26 instructions",
      "factorial(5): 49 instructions",
    ]);
  });
  it("counts the fibonacci machine's runs from 5 to 281", () => {
    const counts = [0, 1, 2, 3, 6].map((n) => {
      const counting = makeCountingMachine(["n", "continue", "val"], fibSimController);
      expectOk(setRegisterContents(counting.machine, "n", n));
      expectOk(counting.machine.start());
      return counting.instructionCount();
    });
    expect(counts).toEqual([5, 5, 28, 51, 281]);
  });
  it("answers the print-and-reset message with the count and zeroes it", () => {
    const result = printAndReset();
    expect(result.printed).toBe(27);
    expect(result.after).toBe(0);
    expect(result.transcript).toEqual(["27"]);
  });
  it("keeps the shared gcd controller's count stable across machines", () => {
    const counting = makeCountingMachine(["a", "b", "t"], gcdController);
    expectOk(setRegisterContents(counting.machine, "a", 206));
    expectOk(setRegisterContents(counting.machine, "b", 40));
    expectOk(counting.machine.start());
    expect(counting.instructionCount()).toBe(26);
  });
});
