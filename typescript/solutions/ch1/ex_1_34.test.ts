// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { f, selfApplicationDiagnostic, workingCalls } from "./ex_1_34.js";

describe("exercise 1.34", () => {
  it("the two working calls land on 4 and 6", () => {
    expect(workingCalls()).toStrictEqual([4, 6]);
  });

  it("f applies its argument to 2", () => {
    expect(f((z: number): number => z + 7)).toBe(9);
  });

  it("the recorded rejection is the checker's TS2345 diagnostic", () => {
    const diagnostic = selfApplicationDiagnostic();
    expect(diagnostic).toContain("error TS2345");
    expect(diagnostic).toContain("is not assignable to parameter of type '(n: number) => number'");
    expect(diagnostic).toContain("Type 'number' is not assignable to type '(n: number) => number'");
  });
});
