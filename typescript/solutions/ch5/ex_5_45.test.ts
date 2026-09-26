// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_45 } from "./ex_5_45.js";

describe("exercise 5.45", () => {
  it("reproduces the book's factorial stack numbers", () => {
    const rows = ex_5_45();
    expect(rows[0]).toContain("interpreted 144/28");
    expect(rows[0]).toContain("compiled 31/14");
    expect(rows[0]).toContain("special-purpose 8/8");
    expect(rows[1]).toContain("n=10");
  });
});
