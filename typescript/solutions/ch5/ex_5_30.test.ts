// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_30 } from "./ex_5_30.ts";

describe("exercise 5.30 error signaling inside the evaluator", () => {
  it("traps division by zero as a typed error", () => {
    expect(ex_5_30().divisionError).not.toBeNull();
  });
  it("traps the bad member access as a typed error", () => {
    expect(ex_5_30().memberError).not.toBeNull();
  });
  it("leaves a correct program untouched", () => {
    const clean = ex_5_30().clean;
    expect(clean.some((line) => line.includes("120"))).toBe(true);
  });
});
