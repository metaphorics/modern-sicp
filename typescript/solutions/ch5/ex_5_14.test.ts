// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  ex_5_14,
  factorialMachineFactorial,
  factorialStackStatistics,
  measuredFactorial,
} from "./ex_5_14.ts";

describe("exercise 5.14 factorial stack statistics", () => {
  it("each level saves continue and n once: exactly 2(n - 1)", () => {
    const table = factorialStackStatistics();
    expect(table.map((row) => row.pushes)).toEqual([0, 2, 4, 6, 8, 10]);
    expect(table.map((row) => row.maxDepth)).toEqual([0, 2, 4, 6, 8, 10]);
  });
  it("the printed message and the read counters agree", () => {
    const measured = measuredFactorial(5);
    expect(measured.printedLine).toBe("(total-pushes = 8 maximum-depth = 8)");
    expect(measured.pushes).toBe(8);
    expect(measured.maxDepth).toBe(8);
  });
  it("the measured machine is the factorial machine", () => {
    expect(factorialMachineFactorial(5)).toBe(120);
    expect(ex_5_14().length).toBe(8);
  });
});
